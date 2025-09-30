use regex::Regex;

use crate::utils::config::AdFilterConfig;
use super::segment::Segment;

#[derive(Debug, Clone)]
pub struct Filter {
    pub url_patterns: Vec<Regex>,
    pub main_size_index: i32,
}

impl Filter {
    pub fn new(config: &AdFilterConfig) -> Self {
        Self {
            url_patterns: config
                .url_patterns
                .iter()
                .map(|pat| Regex::new(pat).unwrap())
                .collect(),
            main_size_index: config.main_size_index,
        }
    }

    pub fn is_ad_by_url(&self, url: &str) -> bool {
        self.url_patterns.iter().any(|pat| pat.is_match(url))
    }

    pub fn update_is_ad_by_size(&self, segments: &mut [Segment]) {
        // TODO
    }
}
