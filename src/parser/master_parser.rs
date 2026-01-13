use crate::error::M3u8Error;
use crate::parser::ContentParser;
use crate::types::{M3u8Playlist, M3u8Variant, PlaylistType};
use url::Url;

// 主播放列表解析器 - 负责解析包含多个变体流的主播放列表
pub struct MasterParser {
    content_parser: ContentParser,
}

impl MasterParser {
    pub fn new(ad_filters: Vec<String>) -> Result<Self, M3u8Error> {
        Ok(Self {
            content_parser: ContentParser::new(ad_filters)?,
        })
    }

    // 解析主播放列表内容
    pub fn parse(&self, content: &str, base_url: Option<&Url>) -> Result<M3u8Playlist, M3u8Error> {
        let mut playlist = M3u8Playlist::new(PlaylistType::Master);
        let lines: Vec<&str> = content.lines().collect();

        // 检查是否为有效的 M3U8 文件
        if !lines.first().unwrap_or(&"").starts_with("#EXTM3U") {
            return Err(M3u8Error::ParseError("不是有效的 M3U8 文件".to_string()));
        }

        let mut current_variant_attrs: Option<(Option<u32>, Option<(u32, u32)>, Option<String>)> =
            None;

        for i in 0..lines.len() {
            let line = lines[i].trim();

            if line.is_empty() || line.starts_with("#EXTM3U") {
                continue;
            }

            if line.starts_with("#EXT-X-VERSION:") {
                playlist.version = line
                    .split(':')
                    .nth(1)
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(1);
            } else if line.starts_with("#EXT-X-STREAM-INF:") {
                // 解析变体流属性
                current_variant_attrs = Some(self.content_parser.parse_stream_inf_line(line));
            } else if !line.starts_with('#') && current_variant_attrs.is_some() {
                // 这是变体流的 URL
                if let Some((bandwidth, resolution, codecs)) = current_variant_attrs.take() {
                    let full_url = self.content_parser.build_full_url(line, base_url)?;

                    if !self.content_parser.is_ad_url(&full_url) {
                        let variant = M3u8Variant {
                            url: full_url,
                            bandwidth,
                            resolution,
                            codecs,
                            audio: None,
                            video: None,
                            subtitles: None,
                        };
                        playlist.variants.push(variant);
                    }
                }
            }
        }

        if playlist.variants.is_empty() {
            return Err(M3u8Error::EmptyError("未找到有效的变体流".to_string()));
        }

        Ok(playlist)
    }

    // 获取最佳质量的变体流索引
    pub fn get_best_quality_index(&self, playlist: &M3u8Playlist) -> usize {
        let mut best_index = 0;
        let mut best_bandwidth = 0;

        for (index, variant) in playlist.variants.iter().enumerate() {
            if let Some(bandwidth) = variant.bandwidth {
                if bandwidth > best_bandwidth {
                    best_bandwidth = bandwidth;
                    best_index = index;
                }
            }
        }

        best_index
    }

    // 根据分辨率选择变体流索引
    pub fn get_variant_by_resolution(
        &self,
        playlist: &M3u8Playlist,
        target_width: u32,
        target_height: u32,
    ) -> Option<usize> {
        for (index, variant) in playlist.variants.iter().enumerate() {
            if let Some((width, height)) = variant.resolution {
                if width == target_width && height == target_height {
                    return Some(index);
                }
            }
        }
        None
    }
}
