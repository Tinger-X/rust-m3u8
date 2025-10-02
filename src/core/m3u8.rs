use reqwest::{Proxy, blocking::Client};
use std::time::Duration;
use url::Url;

use super::filter::Filter;
use super::segment::Segment;
use crate::utils::common::Funcs;
use crate::utils::config::AppConfig;
use crate::utils::errors::{M3u8Error, Result};
use crate::utils::proxy::Proxies;
use crate::utils::tmp_dir::TempDir;
use crate::{error_fmt, warn_fmt};

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
impl M3U8 {
    async fn read_m3u8_content(&self, src: &str, is_online: bool) -> Result<String> {
        let m3u8_content;
        if is_online {
            let client = self.get_client()?;
            let response = client.get(src).send()?;
            m3u8_content = response.text()?;
        } else {
            m3u8_content = std::fs::read_to_string(src)?;
        }
        let m3u8_content = m3u8_content.trim();
        if !m3u8_content.starts_with("#EXTM3U") || !m3u8_content.ends_with("#EXT-X-ENDLIST") {
            return Err(M3u8Error::M3U8Parse("Not a valid M3U8 file".to_string()));
        }
        Ok(m3u8_content.to_string())
    }

    async fn parse_segments(
        &mut self,
        src: &str,
        m3u8_content: &str,
        is_online: bool,
    ) -> Result<()> {
        for (index, &line) in m3u8_content
            .lines()
            .collect::<Vec<&str>>()
            .iter()
            .enumerate()
        {
            let trimmed = line.trim();
            if trimmed.is_empty()
                || trimmed.starts_with("#")
                || (!trimmed.ends_with(".ts") && !trimmed.ends_with(".m4s"))
            {
                continue;
            }
            let mut segment = Segment::new(trimmed.to_string());
            let is_seg_online = Funcs::is_online_resource(trimmed);
            if !is_seg_online {
                if !is_online {
                    error_fmt!("本地M3U8文件仅支持绝对HTTP/HTTPS的TS文件");
                }
                segment.url = Url::parse(src)?.join(trimmed)?.to_string();
            }
            segment.is_ad = self.filter.is_ad_by_url(&segment.url);
            if segment.is_ad {
                self.ads += 1;
            } else {
                self.need_downloads += 1;
                segment.is_ok = self.tmp_dir.load(&index, &mut segment.data).await;
            }
            self.segments.push(segment);
        }
        Ok(())
    }

    fn get_client(&self) -> Result<Client> {
        // 创建HTTP客户端
        let client_builder = Client::builder()
            .timeout(Duration::from_secs(30)) // 设置超时时间
            .connect_timeout(Duration::from_secs(10)); // 设置连接超时时间

        let client = match self.proxies.select() {
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

    async fn download_one(&mut self, index: usize) -> Result<()> {
        for attempt in 0..=self.config.system.retry {
            let client = self.get_client()?;

            match self.segments[index].download(&client).await {
                Ok(_) => {
                    self.tmp_dir.write(&index, &self.segments[index].data);
                    self.downloaded += 1;
                    return Ok(());
                }
                Err(_) => {
                    if attempt == self.config.system.retry {
                        warn_fmt!(
                            "片段下载失败：{}， 重试次数 {}",
                            &self.segments[index].url,
                            attempt
                        );
                        self.errors += 1;
                    }
                }
            }

            std::thread::sleep(Duration::from_secs(2 * (attempt as u64 + 1)));
        }

        return Err(M3u8Error::DownloadFailed(format!(
            "下载片段 {} 失败",
            &self.segments[index].url
        )));
    }
}

impl M3U8 {
    pub fn new(config: &AppConfig) -> Self {
        Self {
            segments: Vec::new(),
            ads: 0,
            errors: 0,
            need_downloads: 0,
            downloaded: 0,
            config: config.clone(),
            filter: Filter::new(&config.filters),
            tmp_dir: TempDir::new(),
            proxies: Proxies::new(),
        }
    }

    pub async fn parse(&mut self, src: &str) -> Result<()> {
        self.proxies.init(&self.config.system.proxies);

        let is_online = Funcs::is_online_resource(src);
        let m3u8_content = self.read_m3u8_content(src, is_online).await?;

        self.tmp_dir.init(&m3u8_content);
        self.parse_segments(src, &m3u8_content, is_online).await?;

        // 解析媒体播放列表
        Ok(())
    }

    /// 下载所有片段
    pub async fn download(&mut self) -> Vec<Result<()>> {
        // 创建线程池
        let pool = match rayon::ThreadPoolBuilder::new()
            .num_threads(self.config.system.workers as usize)
            .build()
        {
            Ok(p) => p,
            Err(e) => {
                return vec![Err(M3u8Error::ThreadError(format!(
                    "创建线程池失败: {}",
                    e
                )))];
            }
        };

        // 并行下载所有视频
        pool.install(|| {
            self.segments
                .iter()
                .enumerate()
                .map(async |(index, segment)| {
                    self.download_one(index)
                        .await
                        .map_err(|e| format!("下载失败 {} (索引 {}): {}", segment.url, index, e))
                })
                .collect()
        })
    }
}
