use rand::Rng;

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

impl ProxyConfig {
    pub fn new() -> Self {
        Self {
            proxies: Vec::new(),
            total_weight: 0,
        }
    }

    pub fn from_args(proxy_args: &[String]) -> Result<Self, String> {
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
            config.add_proxy(url, weight);
        }

        if config.proxies.is_empty() {
            return Err("未找到有效的代理配置".to_string());
        }

        Ok(config)
    }

    pub fn add_proxy(&mut self, url: String, weight: u32) {
        self.proxies.push(ProxyInfo { url, weight });
        self.total_weight += weight;
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
