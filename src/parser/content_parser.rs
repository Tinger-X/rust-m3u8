use crate::error::M3u8Error;
use crate::types::PlaylistType;
use regex::Regex;
use url::Url;

// M3U8 内容解析器 - 负责解析播放列表内容
pub struct ContentParser {
    ad_filters: Vec<Regex>,
}

impl ContentParser {
    pub fn new(ad_filters: Vec<String>) -> Result<Self, M3u8Error> {
        let compiled_filters: Result<Vec<Regex>, _> = ad_filters
            .iter()
            .map(|pattern| Regex::new(pattern))
            .collect();

        let compiled_filters = compiled_filters.map_err(|e| M3u8Error::RegexError(e))?;

        Ok(Self {
            ad_filters: compiled_filters,
        })
    }

    // 解析播放列表内容并确定类型
    pub fn parse_content_type(&self, content: &str) -> PlaylistType {
        let lines: Vec<&str> = content.lines().collect();

        // 检查是否为有效的 M3U8 文件
        if !lines.first().unwrap_or(&"").starts_with("#EXTM3U") {
            return PlaylistType::Unknown;
        }

        // 检查是否包含变体流信息（主播放列表特征）
        for line in &lines {
            if line.starts_with("#EXT-X-STREAM-INF:") || line.contains("BANDWIDTH") {
                return PlaylistType::Master;
            }
        }

        // 检查是否包含片段信息（媒体播放列表特征）
        for line in &lines {
            if line.starts_with("#EXTINF:") {
                return PlaylistType::Media;
            }
        }

        PlaylistType::Unknown
    }

    // 检查 URL 是否为广告
    pub fn is_ad_url(&self, url: &str) -> bool {
        self.ad_filters.iter().any(|regex| regex.is_match(url))
    }

    // 构建完整的 URL
    pub fn build_full_url(&self, url: &str, base_url: Option<&Url>) -> Result<String, M3u8Error> {
        if url.starts_with("http") {
            Ok(url.to_string())
        } else if let Some(base) = base_url {
            Ok(base.join(url)?.to_string())
        } else {
            Err(M3u8Error::ParseError(
                "视频片段为相对URL，请手动配置 base_url".to_string(),
            ))
        }
    }

    // 解析 EXTINF 行
    pub fn parse_extinf_line(&self, line: &str) -> (f64, Option<String>) {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() < 2 {
            return (0.0, None);
        }

        let content = parts[1];
        let duration_part = content.split(',').next().unwrap_or("0.0");
        let title = content.split(',').nth(1).map(|s| s.trim().to_string());

        let duration = duration_part.parse().unwrap_or(0.0);
        (duration, title)
    }

    // 解析 EXT-X-STREAM-INF 行
    pub fn parse_stream_inf_line(
        &self,
        line: &str,
    ) -> (Option<u32>, Option<(u32, u32)>, Option<String>) {
        let mut bandwidth = None;
        let mut resolution = None;
        let mut codecs = None;

        // 解析属性
        let attributes: Vec<&str> = line.split(',').collect();
        for attr in attributes {
            if let Some(bw) = attr.strip_prefix("BANDWIDTH=") {
                bandwidth = bw.parse().ok();
            } else if let Some(res) = attr.strip_prefix("RESOLUTION=") {
                let parts: Vec<&str> = res.split('x').collect();
                if parts.len() == 2 {
                    if let (Ok(width), Ok(height)) = (parts[0].parse(), parts[1].parse()) {
                        resolution = Some((width, height));
                    }
                }
            } else if let Some(c) = attr.strip_prefix("CODECS=") {
                codecs = Some(c.trim_matches('"').to_string());
            }
        }

        (bandwidth, resolution, codecs)
    }
}
