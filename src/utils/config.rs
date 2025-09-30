use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::errors::Result;

/// 系统配置
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct SystemConfig {
    pub workers: u32,
    pub retry: u32,
    pub proxies: Vec<(String, u32)>,
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            workers: 10,
            retry: 3,
            proxies: Vec::new(),
        }
    }
}

/// 广告过滤配置
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct AdFilterConfig {
    pub url_patterns: Vec<String>,
    pub main_size_index: i32,
}

impl Default for AdFilterConfig {
    fn default() -> Self {
        Self {
            url_patterns: Vec::new(),
            main_size_index: -1,
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
    pub fn parse(src: &Option<String>) -> Result<Self> {
        let config = match src {
            Some(path) => {
                let content = std::fs::read_to_string(path)?;
                let mut config: AppConfig = toml::from_str(&content)?;
                if config.system.workers == 0 {
                    eprintln!("警告：workers 配置为 0，已设置为默认值 10");
                    config.system.workers = 10;
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
