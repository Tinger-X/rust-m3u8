use rand::distr::{Distribution, weighted::WeightedIndex};

use crate::{trace_fmt, debug_fmt};

#[derive(Debug, Clone)]
pub struct Proxies {
    pub proxies: Vec<String>,
    dist: Option<WeightedIndex<u32>>,
}

impl Proxies {
    pub fn parse(proxies: &Vec<(String, u32)>) -> Self {
        trace_fmt!("开始解析代理");
        let valid_proxies: Vec<_> = proxies
            .into_iter()
            .filter(|(_, weight)| *weight > 0)
            .collect();
        debug_fmt!("有效代理数: {}", valid_proxies.len());
        if valid_proxies.is_empty() {
            return Self {
                proxies: Vec::new(),
                dist: None,
            };
        }

        let weights: Vec<_> = valid_proxies.iter().map(|(_, w)| *w).collect();
        let proxies = valid_proxies
            .into_iter()
            .map(|(url, _)| url.clone())
            .collect();
        let dist = WeightedIndex::new(&weights).ok();
        Self {
            proxies,
            dist,
        }
    }
    
    /// 根据权重随机选择一个代理
    pub fn select(&self) -> Option<&str> {
        let dist = self.dist.as_ref()?;
        let mut rng = rand::rng();
        let idx = dist.sample(&mut rng);
        self.proxies.get(idx).map(String::as_str)
    }
}
