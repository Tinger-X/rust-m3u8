use bytes::Bytes;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{Proxy, blocking::Client};
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;

use super::filter::Filter;
use super::segment::Segment;
use crate::utils::common::Funcs;
use crate::utils::config::AppConfig;
use crate::utils::errors::{M3u8Error, Result};
use crate::utils::proxy::Proxies;
use crate::utils::tmp_dir::TempDir;
use crate::{debug_fmt, error_fmt, trace_fmt, warn_fmt};

#[derive(Debug, Clone)]
pub struct M3U8 {
    pub segments: Vec<Segment>,
    pub ads: u32,
    pub errors: u32,
    pub need_downloads: u32,
    pub downloaded: u32,
    config: AppConfig,
    filter: Filter,
    tmp_dir: TempDir,
    proxies: Proxies,
}

fn read_m3u8_content(src: &str, proxies: &Proxies) -> Result<String> {
    trace_fmt!("开始读取M3U8文件: {}", src);
    let m3u8_content;
    if Funcs::is_online_resource(src) {
        let client = get_client(proxies)?;
        let response = client.get(src).send()?;
        m3u8_content = response.text()?;
    } else {
        m3u8_content = std::fs::read_to_string(src)?;
    }
    let m3u8_content = m3u8_content.trim();
    if !m3u8_content.starts_with("#EXTM3U") || !m3u8_content.ends_with("#EXT-X-ENDLIST") {
        return Err(M3u8Error::M3U8Parse("Not a valid M3U8 file".to_string()));
    }
    debug_fmt!("M3U8内容长度: {}", m3u8_content.len());
    Ok(m3u8_content.to_string())
}

fn get_client(proxies: &Proxies) -> Result<Client> {
    // 创建HTTP客户端
    let client_builder = Client::builder()
        .timeout(Duration::from_secs(30)) // 设置超时时间
        .connect_timeout(Duration::from_secs(10)); // 设置连接超时时间

    let client = match proxies.select() {
        Some(proxy_url) => match Proxy::all(proxy_url) {
            Ok(proxy) => client_builder.proxy(proxy).build()?,
            Err(e) => {
                warn_fmt!("代理配置错误 {}: {}", proxy_url, e);
                client_builder.build()?
            }
        },
        None => client_builder.build()?,
    };
    Ok(client)
}

impl M3U8 {
    fn parse_segments(&mut self, m3u8_content: &str) -> Result<()> {
        for (index, &line) in m3u8_content
            .lines()
            .collect::<Vec<&str>>()
            .iter()
            .enumerate()
        {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("#") {
                continue;
            }
            let mut segment = Segment::new(trimmed.to_string());
            let is_seg_online = Funcs::is_online_resource(trimmed);
            if !is_seg_online {
                match self.config.system.base_url.as_deref() {
                    Some(base_url) => {
                        segment.url = format!("{}/{}", base_url, trimmed.trim_start_matches('/'));
                    }
                    None => {
                        error_fmt!("TS片段 {} 不是绝对URL，且未配置 base_url", trimmed);
                    }
                }
            }
            segment.is_ad = self.filter.is_ad_by_url(&segment.url);
            if segment.is_ad {
                self.ads += 1;
            } else {
                segment.is_ok = self.tmp_dir.load(&index, &mut segment.data);
                if !segment.is_ok {
                    self.need_downloads += 1;
                }
            }
            self.segments.push(segment);
        }
        Ok(())
    }

    fn download_one(&mut self, index: usize) -> Result<()> {
        for attempt in 0..=self.config.system.retry {
            let client = get_client(&self.proxies)?;

            match self.segments[index].download(&client) {
                Ok(_) => {
                    self.tmp_dir.write(&index, &self.segments[index].data);
                    return Ok(());
                }
                Err(e) => {
                    if attempt == self.config.system.retry {
                        warn_fmt!("index.[{}]下载失败：{}", &index, e);
                    }
                }
            }

            std::thread::sleep(Duration::from_secs(2 * (attempt as u64 + 1)));
        }

        return Err(M3u8Error::DownloadFailed(format!(
            "下载片段 {} 失败",
            &index
        )));
    }

    fn _decode_video_size(&mut self) {
        for segment in self.segments.iter_mut() {
            if !segment.is_ad && segment.is_ok {
                match segment._decode_resolution() {
                    Ok(_) => {}
                    Err(e) => {
                        error_fmt!("解析片段分辨率失败：{}", e);
                    }
                }
            }
        }
    }
}

impl M3U8 {
    pub fn parse(src: &str, config: &AppConfig) -> Result<Self> {
        trace_fmt!("开始解析M3U8文件: {}", src);
        let proxies = Proxies::parse(&config.system.proxies);
        let m3u8_content = read_m3u8_content(src, &proxies)?;

        let mut m3u8 = Self {
            segments: Vec::new(),
            ads: 0,
            errors: 0,
            need_downloads: 0,
            downloaded: 0,
            config: config.clone(),
            filter: Filter::parse(&config.filters),
            tmp_dir: TempDir::parse(&m3u8_content),
            proxies: proxies,
        };

        m3u8.parse_segments(&m3u8_content)?;
        Ok(m3u8)
    }

    /// 下载所有片段
    pub fn download(&mut self) {
        // 创建进度条
        let total_segments = self.need_downloads as u64;
        let pb = ProgressBar::new(total_segments);

        // 设置进度条样式，使用固定的变量而不是动态消息
        let style = ProgressStyle::with_template("下载中：[{bar:50}] {percent:.2f}%, {msg}")
            .unwrap()
            .progress_chars("=> ");
        pb.set_style(style);

        // 使用原子变量来处理并发计数
        let downloaded = AtomicU32::new(0);
        let errors = AtomicU32::new(0);
        let need_downloads = self.need_downloads;

        // 创建线程池
        let pool = match rayon::ThreadPoolBuilder::new()
            .num_threads(self.config.system.workers as usize)
            .build()
        {
            Ok(p) => p,
            Err(e) => {
                error_fmt!("创建线程池失败: {}", e);
                return;
            }
        };

        // 预先过滤出需要下载的片段
        let segments_to_download: Vec<_> = (0..self.segments.len())
            .filter(|&index| !self.segments[index].is_ad && !self.segments[index].is_ok)
            .collect();

        // 并行下载所有视频
        pool.scope(|s| {
            for &index in &segments_to_download {
                let m3u8_clone = self.clone();
                let pb = pb.clone();
                let downloaded = &downloaded;
                let errors = &errors;
                let need_downloads = need_downloads;

                s.spawn(move |_| {
                    let mut cloned = m3u8_clone;
                    match cloned.download_one(index) {
                        Ok(_) => {
                            // 原子递增下载计数
                            let current_downloaded = downloaded.fetch_add(1, Ordering::SeqCst) + 1;
                            let current_errors = errors.load(Ordering::SeqCst);

                            // 更新进度条位置
                            pb.set_position(current_downloaded as u64);

                            // 手动更新消息，以显示所有需要的信息
                            pb.set_message(format!(
                                "{}/{}, 失败: {}",
                                current_downloaded, need_downloads, current_errors
                            ));
                        }
                        Err(_) => {
                            // 原子递增错误计数
                            let current_errors = errors.fetch_add(1, Ordering::SeqCst) + 1;
                            let current_downloaded = downloaded.load(Ordering::SeqCst);

                            // 更新进度条消息
                            pb.set_message(format!(
                                "{}/{}, 失败: {}",
                                current_downloaded, need_downloads, current_errors
                            ));
                        }
                    }
                });
            }
        });

        // 更新最终计数到实例变量
        self.downloaded += downloaded.load(Ordering::SeqCst);
        self.errors += errors.load(Ordering::SeqCst);

        // 完成进度条
        pb.finish_with_message("下载完成");
    }

    pub fn filter_ads_by_size(&mut self) {
        if !self.config.filters.resolution {
            return;
        }
        error_fmt!("当前不支持根据视频分辨率过滤广告");
        // TODO: 实现根据视频分辨率过滤广告
        // self._decode_video_size();
        // self.filter._update_is_ad_by_size(&mut self.segments);
    }

    /// 将所有片段合并到指定文件
    pub fn merge_to_file(&self, output_path: &str) -> Result<()> {
        // 创建输出文件
        let mut output_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(output_path)?;

        // 按顺序写入所有非广告片段
        for (index, segment) in self.segments.iter().enumerate() {
            if !segment.is_ad {
                // 先尝试从内存中获取数据
                if segment.is_ok {
                    output_file.write_all(&segment.data)?;
                } else {
                    // 如果内存中没有，尝试从临时文件读取
                    let mut data = Bytes::new();
                    if self.tmp_dir.load(&index, &mut data) {
                        output_file.write_all(&data)?;
                    } else {
                        warn_fmt!("无法获取片段 {} 的数据，跳过合并", index);
                    }
                }
            }
        }

        output_file.flush()?;
        Ok(())
    }

    /// 清理临时文件
    pub fn cleanup(&mut self) -> Result<()> {
        self.tmp_dir.cleanup()
    }
}
