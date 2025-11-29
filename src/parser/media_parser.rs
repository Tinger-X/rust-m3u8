use crate::error::M3u8Error;
use crate::parser::ContentParser;
use crate::types::{M3u8Playlist, M3u8Segment, PlaylistType};
use url::Url;

// 媒体播放列表解析器 - 负责解析包含具体片段的媒体播放列表
pub struct MediaParser {
    content_parser: ContentParser,
}

impl MediaParser {
    pub fn new(ad_filters: Vec<String>) -> Result<Self, M3u8Error> {
        Ok(Self {
            content_parser: ContentParser::new(ad_filters)?,
        })
    }

    // 解析媒体播放列表内容
    pub fn parse(&self, content: &str, base_url: Option<&Url>) -> Result<M3u8Playlist, M3u8Error> {
        let mut playlist = M3u8Playlist::new(PlaylistType::Media);
        let lines: Vec<&str> = content.lines().collect();

        // 检查是否为有效的 M3U8 文件
        if !lines.first().unwrap_or(&"").starts_with("#EXTM3U") {
            return Err(M3u8Error::ParseError("不是有效的 M3U8 文件".to_string()));
        }

        let mut sequence = 0;
        let mut ads_count = 0;
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();

            if line.is_empty() {
                i += 1;
                continue;
            }

            if line.starts_with("#EXT-X-VERSION:") {
                playlist.version = line
                    .split(':')
                    .nth(1)
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(1);
            } else if line.starts_with("#EXT-X-TARGETDURATION:") {
                playlist.target_duration = line
                    .split(':')
                    .nth(1)
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(0.0);
            } else if line.starts_with("#EXT-X-PLAYLIST-TYPE:") {
                let playlist_type = line.split(':').nth(1).unwrap_or("");
                playlist.is_live = playlist_type != "VOD";
            } else if line.starts_with("#EXT-X-MEDIA-SEQUENCE:") {
                playlist.media_sequence = line
                    .split(':')
                    .nth(1)
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(0);
            } else if line.starts_with("#EXT-X-DISCONTINUITY-SEQUENCE:") {
                playlist.discontinuity_sequence = line
                    .split(':')
                    .nth(1)
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(0);
            } else if line.starts_with("#EXT-X-ENDLIST") {
                playlist.is_live = false;
            } else if line.starts_with("#EXTINF:") {
                // 解析片段信息
                let (duration, title) = self.content_parser.parse_extinf_line(line);

                // 下一行应该是 URL
                if i + 1 < lines.len() {
                    let segment_url = lines[i + 1].trim();
                    if !segment_url.starts_with('#') && !segment_url.is_empty() {
                        let full_url = self.content_parser.build_full_url(segment_url, base_url)?;

                        // 检查是否匹配广告过滤规则
                        let is_ad = self.content_parser.is_ad_url(&full_url);
                        if is_ad {
                            ads_count += 1;
                        } else {
                            let segment = M3u8Segment {
                                url: full_url,
                                duration,
                                sequence,
                                title,
                                byte_range: None,
                            };
                            playlist.segments.push(segment);
                        }
                        sequence += 1;
                        i += 1; // 跳过 URL 行
                    }
                }
            } else if line.starts_with("#EXT-X-BYTERANGE:") {
                // 处理字节范围（需要与前面的片段关联）
                if let Some(last_segment) = playlist.segments.last_mut() {
                    if let Some(byte_range) = self.parse_byte_range(line) {
                        last_segment.byte_range = Some(byte_range);
                    }
                }
            }

            i += 1;
        }

        playlist.ads_count = ads_count;

        if playlist.segments.is_empty() {
            return Err(M3u8Error::ParseError("未找到有效的视频片段".to_string()));
        }

        Ok(playlist)
    }

    // 解析字节范围
    fn parse_byte_range(&self, line: &str) -> Option<(usize, usize)> {
        let content = line.strip_prefix("#EXT-X-BYTERANGE:")?;
        let parts: Vec<&str> = content.split('@').collect();

        if parts.len() == 1 {
            // 只有长度
            let length = parts[0].parse().ok()?;
            Some((0, length))
        } else if parts.len() == 2 {
            // 长度@偏移量
            let length = parts[0].parse().ok()?;
            let offset = parts[1].parse().ok()?;
            Some((offset, length))
        } else {
            None
        }
    }
}
