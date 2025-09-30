use reqwest;
use url::Url;

use crate::utils::common::Funcs;
use crate::utils::config::AppConfig;
use crate::utils::tmp_dir::TempDir;
use crate::utils::errors::{M3u8Error, Result};
use super::segment::Segment;
use super::filter::Filter;

#[derive(Debug, Clone)]
pub struct M3U8 {
    pub segments: Vec<Segment>,
    pub ads: u32,
    pub errors: u32,
    pub need_downloads: u32,
    pub downloaded: u32,
    config: AppConfig,
    filter: Filter,
    tmp_dir: TempDir,
}

impl M3U8 {
    pub fn new(config: &AppConfig) -> Self {
        Self {
            segments: Vec::new(),
            ads: 0,
            errors: 0,
            need_downloads: 0,
            downloaded: 0,
            config: config.clone(),
            filter: Filter::new(&config.filters),
            tmp_dir: TempDir::new(),
        }
    }

    /// 创建新的视频
    pub async fn parse(&mut self, src: &str) -> Result<()> {
        let m3u8_content;
        let is_m3u8_online = Funcs::is_online_resource(src);
        if is_m3u8_online {
            let response = reqwest::Client::new().get(src).send().await?;
            m3u8_content = response.text().await?;
        } else {
            m3u8_content = std::fs::read_to_string(src)?;
        }
        let m3u8_content = m3u8_content.trim();
        if !m3u8_content.starts_with("#EXTM3U") || !m3u8_content.ends_with("#EXT-X-ENDLIST") {
            return Err(M3u8Error::M3U8Parse("Not a valid M3U8 file".to_string()));
        }
        self.tmp_dir.init(&m3u8_content)?;

        // 解析媒体播放列表
        for &line in m3u8_content.lines().collect::<Vec<&str>>().iter() {
            let trimmed = line.trim();
            if trimmed.is_empty()
                || trimmed.starts_with("#")
                || !trimmed.ends_with(".ts")
                || !trimmed.ends_with(".m4s")
            {
                continue;
            }
            let mut segment = Segment::new(trimmed.to_string());
            let is_seg_online = Funcs::is_online_resource(trimmed);
            if !is_seg_online {
                if !is_m3u8_online {
                    return Err(M3u8Error::InvalidUrl(
                        "Local m3u8 file without absolute online url is not allowed".to_string(),
                    ));
                }
                segment.url = Url::parse(src)?.join(trimmed)?.to_string();
            }
            segment.is_ad = self.filter.is_ad_by_url(&segment.url);
            if segment.is_ad {
                self.ads += 1;
            } else {
                self.need_downloads += 1;
            }
            self.segments.push(segment);
        }
        Ok(())
    }
}
