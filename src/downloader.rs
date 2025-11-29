use crate::error::M3u8Error;
use crate::merger::VideoMerger;
use crate::parser::nested_parser::NestedParser;
use crate::proxy::ProxyConfig;
use crate::types::M3u8Segment;
use crate::types::NestedM3u8;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, ACCEPT, ACCEPT_ENCODING, USER_AGENT};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::sync::Semaphore;

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

fn format_size(size: u64, suffix: Option<&str>) -> String {
    let suffix = suffix.unwrap_or("");
    if size >= 1024 * 1024 * 1024 {
        format!(
            "{:.2}GB{}",
            size as f64 / (1024.0 * 1024.0 * 1024.0),
            suffix
        )
    } else if size >= 1024 * 1024 {
        format!("{:.2}MB{}", size as f64 / (1024.0 * 1024.0), suffix)
    } else if size >= 1024 {
        format!("{:.2}KB{}", size as f64 / 1024.0, suffix)
    } else {
        format!("{}B{}", size, suffix)
    }
}

fn create_client_pool(
    proxy_config: &Option<ProxyConfig>,
    pool_size: usize,
) -> Result<Arc<Vec<reqwest::Client>>, M3u8Error> {
    let mut clients = Vec::with_capacity(pool_size);

    for _ in 0..pool_size {
        let client_builder = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .pool_max_idle_per_host(10)
            .pool_idle_timeout(Duration::from_secs(30));

        let client = if let Some(proxy_config) = proxy_config {
            // 如果有代理配置，为每个客户端随机选择一个代理
            if let Some(proxy_url) = proxy_config.get_random_proxy() {
                let proxy = reqwest::Proxy::all(proxy_url)
                    .map_err(|e| M3u8Error::ParseError(format!("代理配置错误: {}", e)))?;
                client_builder.proxy(proxy).build()?
            } else {
                client_builder.build()?
            }
        } else {
            client_builder.build()?
        };

        clients.push(client);
    }

    Ok(Arc::new(clients))
}

pub struct M3u8Downloader {
    url: String,
    output_path: PathBuf,
    temp_dir: PathBuf,
    keep_temp: bool,
    proxy_config: Option<ProxyConfig>,
    max_retries: usize,
    base_url: Option<String>,
    headers: HeaderMap,
    ad_filters: Vec<String>,
    simple: bool,
    client_pool: Arc<Vec<reqwest::Client>>,
    client_semaphore: Arc<Semaphore>,
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
    ) -> Result<Self, M3u8Error> {
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

        // 创建客户端池
        let client_pool = create_client_pool(&proxy_config, concurrent_limit)?;
        let client_semaphore = Arc::new(Semaphore::new(concurrent_limit));

        Ok(Self {
            url,
            output_path,
            temp_dir,
            keep_temp,
            proxy_config,
            max_retries,
            base_url,
            headers,
            ad_filters,
            simple,
            client_pool,
            client_semaphore,
        })
    }

    pub async fn download(&self) -> Result<(), M3u8Error> {
        // 创建临时目录
        fs::create_dir_all(&self.temp_dir).await?;

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

        self.display_playlist_info(&nested);
        let segments = nested
            .get_selected_variant()
            .map(|playlist| &playlist.segments)
            .ok_or_else(|| M3u8Error::ParseError("未找到有效的播放列表片段".to_string()))?;
        self.download_segments(segments).await?;
        let merger = VideoMerger::new();
        if self.simple {
            merger
                .merge_with_rust(&self.temp_dir, &self.output_path, segments)
                .await?;
        } else {
            merger
                .merge_with_ffmpeg(&self.temp_dir, &self.output_path, segments)
                .await?;
        }
        if !self.keep_temp {
            fs::remove_dir_all(&self.temp_dir).await?;
        }

        Ok(())
    }

    fn display_playlist_info(&self, nested: &NestedM3u8) {
        if let Some(selected_playlist) = nested.get_selected_variant() {
            let mut info_parts = Vec::new();
            info_parts.push(format!("📊 {} 个片段", selected_playlist.segments.len()));
            info_parts.push(format!(
                "🕒 {}",
                format_duration(&selected_playlist.segments)
            ));

            if selected_playlist.ads_count > 0 {
                info_parts.push(format!("🚫 广告 {} 个", selected_playlist.ads_count));
            }

            if selected_playlist.is_live {
                info_parts.push("📡 直播流".to_string());
            }

            println!("{}\n", info_parts.join(", "));
        }
    }

    async fn download_segments(&self, segments: &[M3u8Segment]) -> Result<(), M3u8Error> {
        let total_bytes = Arc::new(AtomicU64::new(0));
        let last_update = Arc::new(AtomicU64::new(0));
        let progress_bar = ProgressBar::new(segments.len() as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] |{bar:50.cyan/blue}| {pos}/{len}: {percent}%, ETA: {eta}, {msg}")
                .unwrap()
                .progress_chars("⣿⣷⣶⣦⣤⣄⣀ "),
        );
        progress_bar.set_message("...");

        let progress_bar = Arc::new(progress_bar);
        let total_bytes_clone = Arc::clone(&total_bytes);
        let last_update_clone = Arc::clone(&last_update);
        let speed_bar = Arc::clone(&progress_bar);
        let segment_bar = Arc::clone(&progress_bar);

        // 启动速度更新任务
        let speed_update_handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(1));
            loop {
                interval.tick().await;
                let current_bytes = total_bytes_clone.load(Ordering::Relaxed);
                let last_bytes = last_update_clone.swap(current_bytes, Ordering::Relaxed);

                if last_bytes > 0 {
                    speed_bar.set_message(format_size(current_bytes - last_bytes, Some("/s")));
                }
            }
        });

        // 使用 JoinSet 进行并发下载
        use tokio::task::JoinSet;
        let mut join_set = JoinSet::new();

        let temp_dir = self.temp_dir.clone();
        let max_retries = self.max_retries;
        let client_pool = Arc::clone(&self.client_pool);
        let client_semaphore = Arc::clone(&self.client_semaphore);
        let headers = self.headers.clone();

        // 为每个分片创建下载任务
        for segment in segments.iter() {
            let total_bytes_task = Arc::clone(&total_bytes);
            let progress_bar_task = Arc::clone(&segment_bar);
            let segment_clone = segment.clone();
            let temp_dir_clone = temp_dir.clone();
            let headers_clone = headers.clone();
            let client_pool_clone = Arc::clone(&client_pool);
            let client_semaphore_clone = Arc::clone(&client_semaphore);

            join_set.spawn(async move {
                let result = Self::download_single_segment(
                    &segment_clone,
                    &total_bytes_task,
                    &temp_dir_clone,
                    max_retries,
                    &client_pool_clone,
                    &client_semaphore_clone,
                    &headers_clone,
                )
                .await;

                // 无论成功与否，都更新进度条
                progress_bar_task.inc(1);
                result
            });
        }

        // 等待所有下载任务完成
        let mut download_results = Vec::new();
        while let Some(task_result) = join_set.join_next().await {
            match task_result {
                Ok(download_result) => {
                    download_results.push(download_result);
                }
                Err(join_error) => {
                    // 如果任务本身失败（比如 panic），记录错误
                    download_results.push(Err(M3u8Error::ParseError(format!(
                        "下载任务异常: {}",
                        join_error
                    ))));
                }
            }
        }

        speed_update_handle.abort();

        progress_bar.finish_with_message(format!(
            "✅ 下载完成! 总下载量: {}\n",
            format_size(total_bytes.load(Ordering::Relaxed), None)
        ));

        // 检查所有下载结果
        for result in download_results {
            result?;
        }

        Ok(())
    }

    async fn download_single_segment(
        segment: &M3u8Segment,
        total_bytes: &Arc<AtomicU64>,
        temp_dir: &PathBuf,
        max_retries: usize,
        client_pool: &Arc<Vec<reqwest::Client>>,
        client_semaphore: &Arc<Semaphore>,
        headers: &HeaderMap,
    ) -> Result<(), M3u8Error> {
        let file_name = format!("seg{:06}.ts", segment.sequence);
        let file_path = temp_dir.join(&file_name);
        if file_path.exists() {
            return Ok(());
        }

        let mut retry_count = 0;
        while retry_count < max_retries {
            match Self::try_download_segment(
                &segment.url,
                &file_path,
                total_bytes,
                client_pool,
                client_semaphore,
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
        url: &str,
        file_path: &PathBuf,
        total_bytes: &Arc<AtomicU64>,
        client_pool: &Arc<Vec<reqwest::Client>>,
        client_semaphore: &Arc<Semaphore>,
        headers: &HeaderMap,
    ) -> Result<(), M3u8Error> {
        // 获取客户端信号量许可
        let _permit = client_semaphore
            .acquire()
            .await
            .map_err(|e| M3u8Error::ParseError(format!("获取客户端许可失败: {}", e)))?;

        // 从客户端池中随机选择一个客户端
        let client_index = rand::random::<usize>() % client_pool.len();
        let client = &client_pool[client_index];
        let response = client.get(url).headers(headers.clone()).send().await?;

        if !response.status().is_success() {
            return Err(M3u8Error::ParseError(format!(
                "HTTP 请求失败: {}",
                response.status()
            )));
        }

        let bytes = response.bytes().await?;
        total_bytes.fetch_add(bytes.len() as u64, Ordering::Relaxed);
        let mut file = fs::File::create(file_path).await?;
        file.write_all(&bytes).await?;
        file.flush().await?;
        return Ok(());
    }
}
