use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::args::Cli;
use super::errors::Result;
use super::logger::LogLevel;
use crate::warn_fmt;

/// 系统配置
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct SystemConfig {
    pub workers: u32,
    pub retry: u32,
    pub proxies: Vec<(String, u32)>,
    pub log_level: LogLevel,
    pub base_url: Option<String>,
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            workers: 10,
            retry: 3,
            proxies: Vec::new(),
            log_level: LogLevel::Info,
            base_url: None,
        }
    }
}

/// 广告过滤配置
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct AdFilterConfig {
    pub url_patterns: Vec<String>,
    pub resolution: bool,
}

impl Default for AdFilterConfig {
    fn default() -> Self {
        Self {
            url_patterns: Vec::new(),
            resolution: false,
        }
    }
}

/// 应用程序配置
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct AppConfig {
    /// 系统配置
    pub system: SystemConfig,

    /// HTTP请求头
    pub headers: HashMap<String, String>,

    /// 广告过滤配置
    pub filters: AdFilterConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        let mut headers = HashMap::new();
        headers.insert("User-Agent".to_string(), "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string());
        headers.insert("Accept".to_string(), "*/*".to_string());
        headers.insert(
            "Accept-Encoding".to_string(),
            "gzip, deflate, br".to_string(),
        );
        headers.insert("Connection".to_string(), "keep-alive".to_string());

        Self {
            system: SystemConfig::default(),
            headers: headers,
            filters: AdFilterConfig::default(),
        }
    }
}

impl AppConfig {
    fn inner_parse(src: &Option<String>) -> Result<Self> {
        let config = match src {
            Some(path) => {
                let content = std::fs::read_to_string(path)?;
                let mut config: AppConfig = toml::from_str(&content)?;
                
                if config.system.workers == 0 {
                    warn_fmt!("使用默认 workers: 10");
                    config.system.workers = 10;
                }
                if config.system.retry == 0 {
                    config.system.retry = u32::MAX;
                }
                if config.system.base_url.is_some() {
                    let base_url = config.system.base_url.as_mut().unwrap();
                    if !base_url.starts_with("http://") && !base_url.starts_with("https://") {
                        base_url.insert_str(0, "https://");
                    }
                    config.system.base_url = Some(base_url.trim_end_matches('/').to_string());
                }

                let default = AppConfig::default();
                if config.headers.is_empty() {
                    config.headers = default.headers;
                } else {
                    for (key, value) in default.headers {
                        if !config.headers.contains_key(&key) {
                            config.headers.insert(key, value);
                        }
                    }
                }

                config
            }
            None => AppConfig::default(),
        };
        Ok(config)
    }
}

impl AppConfig {
    pub fn parse(src: &Option<String>) -> Self {
        match Self::inner_parse(src) {
            Ok(config) => config,
            Err(e) => {
                warn_fmt!("解析配置文件失败: {}", e);
                AppConfig::default()
            }
        }
    }

    pub fn accept_cli(&mut self, cli: &Cli) {
        if let Some(ref base_url) = cli.base_url {
            self.system.base_url = Some(base_url.clone());
        }
        for header in &cli.headers {
            if let Some((key, value)) = header.split_once(':') {
                self.headers.insert(key.trim().to_string(), value.trim().to_string());
            } else {
                warn_fmt!("已忽略无法解析的请求头：{}", header);
            }
        }
    }
}
