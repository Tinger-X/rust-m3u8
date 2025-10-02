use rand::distr::{Distribution, weighted::WeightedIndex};

#[derive(Debug, Clone)]
pub struct Proxies {
    proxies: Vec<String>,
    dist: Option<WeightedIndex<u32>>,
}

impl Proxies {
    /// 创建新的代理选择器，过滤掉权重为0的代理，并创建加权分布
    pub fn new() -> Self {
        Self {
            proxies: Vec::new(),
            dist: None,
        }
    }

    /// 初始化的代理选择器，过滤掉权重为0的代理，并创建加权分布
    pub fn init(&mut self, proxies: &Vec<(String, u32)>) {
        let valid_proxies: Vec<_> = proxies
            .into_iter()
            .filter(|(_, weight)| *weight > 0)
            .collect();
        if valid_proxies.is_empty() {
            return;
        }

        let weights: Vec<_> = valid_proxies.iter().map(|(_, w)| *w).collect();
        self.proxies = valid_proxies
            .into_iter()
            .map(|(url, _)| url.clone())
            .collect();
        self.dist = WeightedIndex::new(&weights).ok();
    }
    
    /// 根据权重随机选择一个代理
    pub fn select(&self) -> Option<&str> {
        let dist = self.dist.as_ref()?;
        let mut rng = rand::rng();
        let idx = dist.sample(&mut rng);
        self.proxies.get(idx).map(String::as_str)
    }
}
