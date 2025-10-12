use crate::error::M3u8Error;
use crate::merger::VideoMerger;
use crate::parser::nested_parser::NestedParser;
use crate::proxy::ProxyConfig;
use crate::types::M3u8Segment;
use crate::types::NestedM3u8;
use futures::stream::{self, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, ACCEPT, ACCEPT_ENCODING, USER_AGENT};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tokio::fs;
use tokio::io::AsyncWriteExt;

/// 将秒数转换为人类可读的时长格式
fn format_duration(segments: &[M3u8Segment]) -> String {
    let total_seconds = segments.iter().map(|s| s.duration).sum::<f64>();

    if total_seconds < 60.0 {
        format!("00:00:{:02} s", total_seconds as u32)
    } else if total_seconds < 3600.0 {
        let minutes = (total_seconds / 60.0) as u32;
        let seconds = (total_seconds % 60.0) as u32;
        format!("00:{:02}:{:02} s", minutes, seconds)
    } else {
        let hours = (total_seconds / 3600.0) as u32;
        let minutes = ((total_seconds % 3600.0) / 60.0) as u32;
        let seconds = (total_seconds % 60.0) as u32;
        format!("{:02}:{:02}:{:02} s", hours, minutes, seconds)
    }
}

pub struct M3u8Downloader {
    url: String,
    output_path: PathBuf,
    temp_dir: PathBuf,
    concurrent_limit: usize,
    keep_temp: bool,
    proxy_config: Option<ProxyConfig>,
    max_retries: usize,
    base_url: Option<String>,
    headers: HeaderMap,
    ad_filters: Vec<String>,
    simple: bool,
}

impl M3u8Downloader {
    pub fn new(
        url: String,
        output_path: PathBuf,
        temp_dir: PathBuf,
        concurrent_limit: usize,
        keep_temp: bool,
        proxy_config: Option<ProxyConfig>,
        max_retries: usize,
        base_url: Option<String>,
        custom_headers: Vec<String>,
        ad_filters: Vec<String>,
        simple: bool,
    ) -> Self {
        // 创建默认请求头
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36"));
        headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
        headers.insert(
            ACCEPT_ENCODING,
            HeaderValue::from_static("gzip, deflate, br"),
        );

        // 解析并添加自定义请求头
        for header_str in custom_headers {
            if let Some((key, value)) = header_str.split_once(':') {
                let key = key.trim();
                let value = value.trim();
                if let (Ok(header_name), Ok(header_value)) =
                    (HeaderName::from_str(key), HeaderValue::from_str(value))
                {
                    headers.insert(header_name, header_value);
                }
            }
        }

        Self {
            url,
            output_path,
            temp_dir,
            concurrent_limit,
            keep_temp,
            proxy_config,
            max_retries,
            base_url,
            headers,
            ad_filters,
            simple,
        }
    }

    pub async fn download(&self) -> Result<(), M3u8Error> {
        // 创建临时目录
        fs::create_dir_all(&self.temp_dir).await?;

        // 解析 M3U8 播放列表（支持嵌套）
        // println!("📋 获取并解析 M3U8 播放列表...");
        let parser = NestedParser::new(self.ad_filters.clone())?;

        let nested = if self.url.starts_with("http") {
            parser
                .parse_from_url(&self.url, self.proxy_config.as_ref(), &self.headers)
                .await?
        } else {
            parser
                .parse_from_file(&self.url, self.base_url.as_deref())
                .await?
        };

        // 显示播放列表信息
        self.display_playlist_info(&nested);

        // 获取当前选中的播放列表片段
        let segments = nested
            .get_selected_variant()
            .map(|playlist| &playlist.segments)
            .ok_or_else(|| M3u8Error::ParseError("未找到有效的播放列表片段".to_string()))?;

        // 下载所有片段
        self.download_segments(segments).await?;

        // 合并视频片段
        let merger = VideoMerger::new();
        if self.simple {
            // println!("📝 使用简单合并模式...");
            merger
                .merge_with_rust(&self.temp_dir, &self.output_path, segments)
                .await?;
        } else {
            // println!("🎬 使用 FFmpeg 合并视频片段...");
            merger
                .merge_with_ffmpeg(&self.temp_dir, &self.output_path, segments)
                .await?;
        }

        // 清理临时文件
        if !self.keep_temp {
            // println!("🧹 正在清理临时文件...");
            fs::remove_dir_all(&self.temp_dir).await?;
        }

        Ok(())
    }

    /// 显示播放列表信息（支持嵌套播放列表）
    fn display_playlist_info(&self, nested: &NestedM3u8) {
        if let Some(selected_playlist) = nested.get_selected_variant() {
            // if nested.master_playlist.is_nested() {
            //     println!("🎯 检测到嵌套播放列表（主播放列表）, 📊 可用变体流数量: {}", nested.master_playlist.variants.len());
            //     // 显示变体流信息
            //     for (index, variant) in nested.master_playlist.variants.iter().enumerate() {
            //         let quality_info = if let Some(bandwidth) = variant.bandwidth {
            //             if let Some((width, height)) = variant.resolution {
            //                 format!("{}x{} @ {} kbps", width, height, bandwidth / 1000)
            //             } else {
            //                 format!("{} kbps", bandwidth / 1000)
            //             }
            //         } else {
            //             "未知质量".to_string()
            //         };
            //         let selected_marker = if nested.selected_variant_index == Some(index) {
            //             "✅ 当前选择"
            //         } else {
            //             "  "
            //         };
            //         println!("   {} [{}] {}", selected_marker, index, quality_info);
            //     }
            //     println!();
            // }

            print!(
                "🎥 播放列表类型: {}, 🚫 广告检出数: {}, 📊 共 {} 个视频片段, 🕒 总时长约: {}, ",
                selected_playlist.playlist_type,
                selected_playlist.ads_count,
                selected_playlist.segments.len(),
                format_duration(&selected_playlist.segments),
            );

            if selected_playlist.is_live {
                println!("📡 直播流模式");
            } else {
                println!("🎬 点播流模式");
            }
            println!();
        }
    }

    async fn download_segments(&self, segments: &[M3u8Segment]) -> Result<(), M3u8Error> {
        // 创建基础 HTTP 客户端
        let client = Arc::new(reqwest::Client::builder().build()?);
        let temp_dir = Arc::new(self.temp_dir.clone());

        // 创建增强的进度条
        let progress_bar = ProgressBar::new(segments.len() as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] |{bar:50.cyan/blue}| {pos}/{len}: {percent}%, ETA: {eta}, {msg}")
                .unwrap()
                .progress_chars("█▓░"),
        );
        progress_bar.set_message(format!("{:06}.ts", 0));

        let progress_bar = Arc::new(progress_bar);
        // 使用流来限制并发数量
        let results: Vec<Result<(), M3u8Error>> = stream::iter(segments.iter())
            .map(|segment| {
                let client = Arc::clone(&client);
                let temp_dir = Arc::clone(&temp_dir);
                let progress_bar = Arc::clone(&progress_bar);
                let segment = segment.clone();
                let proxy_config = self.proxy_config.clone();
                let headers = self.headers.clone();

                async move {
                    let result = Self::download_single_segment(
                        &client,
                        &temp_dir,
                        &segment,
                        proxy_config.as_ref(),
                        &headers,
                        self.max_retries,
                    )
                    .await;
                    progress_bar.inc(1);
                    progress_bar.set_message(format!("{:06}.ts", segment.sequence));
                    result
                }
            })
            .buffer_unordered(self.concurrent_limit)
            .collect()
            .await;

        progress_bar.finish_with_message("✅ 下载完成!\n");

        // 检查是否有下载失败的片段
        for result in results {
            result?;
        }

        Ok(())
    }

    async fn download_single_segment(
        client: &reqwest::Client,
        temp_dir: &PathBuf,
        segment: &M3u8Segment,
        proxy_config: Option<&ProxyConfig>,
        headers: &HeaderMap,
        max_retries: usize,
    ) -> Result<(), M3u8Error> {
        let file_name = format!("segment_{:06}.ts", segment.sequence);
        let file_path = temp_dir.join(&file_name);

        // 如果文件已存在，跳过下载
        if file_path.exists() {
            return Ok(());
        }

        let mut retry_count = 0;

        while retry_count < max_retries {
            match Self::try_download_segment(
                client,
                &segment.url,
                &file_path,
                proxy_config,
                headers,
            )
            .await
            {
                Ok(_) => return Ok(()),
                Err(e) => {
                    retry_count += 1;
                    if retry_count >= max_retries {
                        eprintln!(
                            "❌ 下载片段 {} 失败 (重试 {} 次): {}",
                            segment.sequence, max_retries, e
                        );
                        return Err(e);
                    }
                    tokio::time::sleep(tokio::time::Duration::from_millis(
                        1000 * retry_count as u64,
                    ))
                    .await;
                }
            }
        }

        Ok(())
    }

    async fn try_download_segment(
        client: &reqwest::Client,
        url: &str,
        file_path: &PathBuf,
        proxy_config: Option<&ProxyConfig>,
        headers: &HeaderMap,
    ) -> Result<(), M3u8Error> {
        // 如果配置了代理，为这个请求单独选择一个代理
        if let Some(proxy_config) = proxy_config {
            if let Some(proxy_url) = proxy_config.get_random_proxy() {
                let proxy_client = reqwest::Client::builder()
                    .proxy(
                        reqwest::Proxy::http(proxy_url)
                            .map_err(|e| M3u8Error::ParseError(format!("代理配置错误: {}", e)))?,
                    )
                    .build()?;
                let response = proxy_client
                    .get(url)
                    .headers(headers.clone())
                    .send()
                    .await?;

                if !response.status().is_success() {
                    return Err(M3u8Error::ParseError(format!(
                        "HTTP 请求失败: {}",
                        response.status()
                    )));
                }

                let bytes = response.bytes().await?;
                let mut file = fs::File::create(file_path).await?;
                file.write_all(&bytes).await?;
                file.flush().await?;
                return Ok(());
            }
        }

        // 没有代理或代理选择失败时使用默认客户端
        let response = client.get(url).headers(headers.clone()).send().await?;

        if !response.status().is_success() {
            return Err(M3u8Error::ParseError(format!(
                "HTTP 请求失败: {}",
                response.status()
            )));
        }

        let bytes = response.bytes().await?;
        let mut file = fs::File::create(file_path).await?;
        file.write_all(&bytes).await?;
        file.flush().await?;

        Ok(())
    }
}
