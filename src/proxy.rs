use rand::Rng;
use std::time::Duration;
use tokio::time::timeout;

#[derive(Debug, Clone)]
pub struct ProxyInfo {
    pub url: String,
    pub weight: u32,
}

#[derive(Debug, Clone)]
pub struct ProxyConfig {
    proxies: Vec<ProxyInfo>,
    total_weight: u32,
}

async fn is_proxy_valid(proxy_url: &str) -> Result<(), String> {
    let full_url = if proxy_url.starts_with("http://") || proxy_url.starts_with("https://") {
        proxy_url.to_string()
    } else {
        format!("http://{}", proxy_url)
    };

    // 首先测试代理服务器是否可达
    let proxy_url = url::Url::parse(&full_url).map_err(|e| format!("代理URL解析失败: {}", e))?;
    let host = proxy_url.host_str().ok_or("代理URL缺少主机名")?;
    let port = proxy_url.port().unwrap_or(80);

    // 尝试TCP连接到代理服务器
    let tcp_result = timeout(
        Duration::from_secs(5),
        tokio::net::TcpStream::connect((host, port)),
    )
    .await;

    match tcp_result {
        Ok(Ok(_stream)) => {
            // TCP连接成功，继续测试HTTP代理功能
        }
        Ok(Err(e)) => {
            return Err(format!("代理服务器不可达: {}", e));
        }
        Err(_) => {
            return Err("代理服务器连接超时".to_string());
        }
    }

    let proxy = reqwest::Proxy::all(&full_url).map_err(|e| format!("代理配置错误: {}", e))?;
    let client = reqwest::Client::builder()
        .proxy(proxy)
        // .timeout(Duration::from_secs(5))
        .build()
        .map_err(|e| format!("创建代理客户端失败: {}", e))?;

    // 使用一个简单的请求测试代理
    let http_result = timeout(
        Duration::from_secs(5),
        client.get("https://www.baidu.com").send(),
    )
    .await;

    match http_result {
        Ok(Ok(response)) => {
            if response.status().is_success() {
                let text = response
                    .text()
                    .await
                    .map_err(|e| format!("读取响应失败: {}", e))?;

                // 检查响应
                if text.contains("百度一下") {
                    Ok(())
                } else {
                    Err("代理响应格式异常".to_string())
                }
            } else {
                Err(format!("代理响应错误: {}", response.status()))
            }
        }
        Ok(Err(e)) => Err(format!("代理HTTP请求失败: {}", e)),
        Err(_) => Err("代理HTTP请求超时".to_string()),
    }
}

impl ProxyConfig {
    pub fn new() -> Self {
        Self {
            proxies: Vec::new(),
            total_weight: 0,
        }
    }

    pub async fn from_args(proxy_args: &[String]) -> Result<Self, String> {
        let mut config = Self::new();

        for proxy_arg in proxy_args {
            let parts: Vec<&str> = proxy_arg.trim().split(',').collect();
            if parts.len() != 2 {
                return Err(format!(
                    "代理格式错误: {}，应为 'weight,proxy_url'",
                    proxy_arg
                ));
            }

            let weight: u32 = parts[0]
                .parse()
                .map_err(|_| format!("权重解析错误: {}", parts[0]))?;

            let url = parts[1].to_string();
            config.add_proxy(url, weight).await;
        }

        if config.proxies.is_empty() {
            return Err("未找到有效的代理配置".to_string());
        }

        Ok(config)
    }

    pub async fn add_proxy(&mut self, url: String, weight: u32) {
        match is_proxy_valid(&url).await {
            Ok(()) => {
                self.proxies.push(ProxyInfo { url, weight });
                self.total_weight += weight;
            }
            Err(e) => {
                eprintln!("❌ 代理[{}]配置错误: {}", url, e);
                return;
            }
        }
    }

    pub fn get_random_proxy(&self) -> Option<&str> {
        if self.proxies.is_empty() {
            return None;
        }

        let mut rng = rand::thread_rng();
        let mut random_weight = rng.gen_range(0..self.total_weight);

        for proxy in &self.proxies {
            if random_weight < proxy.weight {
                return Some(&proxy.url);
            }
            random_weight -= proxy.weight;
        }

        // 备用方案，返回第一个代理
        Some(&self.proxies[0].url)
    }

    pub fn len(&self) -> usize {
        self.proxies.len()
    }
}
