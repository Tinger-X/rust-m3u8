use regex::Regex;
use std::collections::HashMap;

use super::segment::Segment;
use crate::utils::config::AdFilterConfig;
use crate::{trace_fmt, debug_fmt};

#[derive(Debug, Clone)]
pub struct Filter {
    url_patterns: Vec<Regex>,
    _resolution: bool,
}

impl Filter {
    pub fn parse(config: &AdFilterConfig) -> Self {
        trace_fmt!("开始解析广告过滤配置");
        let url_patterns: Vec<Regex> = config
                .url_patterns
                .iter()
                .map(|pat| Regex::new(pat).unwrap())
                .collect();
        debug_fmt!("广告过滤URL模式数: {}", url_patterns.len());
        Self {
            url_patterns,
            _resolution: config.resolution,
        }
    }

    pub fn is_ad_by_url(&self, url: &str) -> bool {
        self.url_patterns.iter().any(|pat| pat.is_match(url))
    }

    pub fn _update_is_ad_by_size(&self, segments: &mut [Segment]) {
        if !self._resolution {
            return;
        }
        // 统计每个size的出现频次
        let mut frequency_map = HashMap::new();

        for segment in &*segments {
            *frequency_map.entry(segment._size.clone()).or_insert(0) += 1;
        }

        // 找到最高频次
        let max_frequency = frequency_map.values().max().copied().unwrap_or(0);

        // 找出所有具有最高频次的size值，并按首次出现顺序保留第一个
        let mut first_most_frequent_size: Option<Option<(u32, u32)>> = None;

        // 遍历原始数组以确定第一个出现的最高频次size
        for segment in &*segments {
            let count = frequency_map.get(&segment._size).copied().unwrap_or(0);
            if count == max_frequency && first_most_frequent_size.is_none() {
                first_most_frequent_size = Some(segment._size.clone());
                break; // 找到第一个就退出
            }
        }

        // 遍历节点，只保留第一个最高频次的节点的is_ad为false
        if let Some(keep_size) = first_most_frequent_size {
            for segment in segments {
                if segment._size != keep_size {
                    segment.is_ad = true;
                }
            }
        }
    }
}
