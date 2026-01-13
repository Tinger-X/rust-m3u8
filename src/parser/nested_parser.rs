use crate::error::M3u8Error;
use crate::parser::{ContentParser, MasterParser, MediaParser};
use crate::types::{M3u8Playlist, M3u8Segment, NestedM3u8, PlaylistType};
use url::Url;

// 嵌套播放列表解析器 - 负责处理包含主播放列表和多个媒体播放列表的嵌套结构
pub struct NestedParser {
    content_parser: ContentParser,
    master_parser: MasterParser,
    media_parser: MediaParser,
}

impl NestedParser {
    pub fn new(ad_filters: Vec<String>) -> Result<Self, M3u8Error> {
        Ok(Self {
            content_parser: ContentParser::new(ad_filters.clone())?,
            master_parser: MasterParser::new(ad_filters.clone())?,
            media_parser: MediaParser::new(ad_filters)?,
        })
    }

    // 从 URL 解析嵌套播放列表
    pub async fn parse_from_url(
        &self,
        url: &str,
        client: &reqwest::Client,
    ) -> Result<NestedM3u8, M3u8Error> {
        let response = client.get(url).send().await?;
        let content = response.text().await?;

        self.parse_content(&content, Some(url), client).await
    }

    // 从文件解析嵌套播放列表
    pub async fn parse_from_file(
        &self,
        file_path: &str,
        base_url: Option<&str>,
        client: &reqwest::Client,
    ) -> Result<NestedM3u8, M3u8Error> {
        let content = tokio::fs::read_to_string(file_path)
            .await
            .map_err(|e| M3u8Error::IoError(e))?;

        self.parse_content(&content, base_url, &client).await
    }

    // 解析嵌套播放列表内容
    pub async fn parse_content(
        &self,
        content: &str,
        base_url: Option<&str>,
        client: &reqwest::Client,
    ) -> Result<NestedM3u8, M3u8Error> {
        let base_url_obj = base_url.and_then(|url| Url::parse(url).ok());

        // 首先确定播放列表类型
        let playlist_type = self.content_parser.parse_content_type(content);

        match playlist_type {
            PlaylistType::Master => {
                // 这是主播放列表，需要递归解析所有变体流
                self.parse_master_playlist(content, &base_url_obj, client)
                    .await
            }
            PlaylistType::Media => {
                // 这是媒体播放列表，直接包装为嵌套结构
                self.parse_single_media_playlist(content, &base_url_obj)
                    .await
            }
            PlaylistType::Unknown => {
                Err(M3u8Error::ParseError("无法识别的播放列表类型".to_string()))
            }
        }
    }

    // 解析主播放列表及其所有变体流
    async fn parse_master_playlist(
        &self,
        content: &str,
        base_url: &Option<Url>,
        client: &reqwest::Client,
    ) -> Result<NestedM3u8, M3u8Error> {
        let mut nested = NestedM3u8::new();

        // 解析主播放列表
        nested.master_playlist = self.master_parser.parse(content, base_url.as_ref())?;

        // 递归解析所有变体流
        for variant in &nested.master_playlist.variants {
            let media_playlist = self.parse_variant_playlist(&variant.url, client).await?;
            nested.media_playlists.push(media_playlist);
        }

        // 默认选择最佳质量的变体流
        let best_index = self
            .master_parser
            .get_best_quality_index(&nested.master_playlist);
        nested.selected_variant_index = Some(best_index);

        Ok(nested)
    }

    // 解析单个变体流播放列表
    async fn parse_variant_playlist(
        &self,
        url: &str,
        client: &reqwest::Client,
    ) -> Result<M3u8Playlist, M3u8Error> {
        let response = client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(M3u8Error::ParseError(format!(
                "无法获取变体流播放列表: {} - {}",
                url,
                response.status()
            )));
        }

        let content = response.text().await?;
        let base_url_obj = Url::parse(url).ok();

        self.media_parser.parse(&content, base_url_obj.as_ref())
    }

    // 解析单个媒体播放列表并包装为嵌套结构
    async fn parse_single_media_playlist(
        &self,
        content: &str,
        base_url: &Option<Url>,
    ) -> Result<NestedM3u8, M3u8Error> {
        let mut nested = NestedM3u8::new();

        // 解析媒体播放列表
        let media_playlist = self.media_parser.parse(content, base_url.as_ref())?;
        nested.media_playlists.push(media_playlist);
        nested.selected_variant_index = Some(0);

        Ok(nested)
    }

    // 获取当前选中的播放列表的所有片段
    pub fn get_selected_segments<'a>(
        &self,
        nested: &'a NestedM3u8,
    ) -> Option<&'a Vec<M3u8Segment>> {
        nested
            .get_selected_variant()
            .map(move |playlist| &playlist.segments)
    }

    // 获取所有变体流的信息
    pub fn get_variants_info(&self, nested: &NestedM3u8) -> Vec<(usize, String)> {
        nested
            .master_playlist
            .variants
            .iter()
            .enumerate()
            .map(|(index, variant)| {
                let info = if let Some(bandwidth) = variant.bandwidth {
                    if let Some((width, height)) = variant.resolution {
                        format!("{}x{} @ {} kbps", width, height, bandwidth / 1000)
                    } else {
                        format!("{} kbps", bandwidth / 1000)
                    }
                } else {
                    "未知质量".to_string()
                };
                (index, info)
            })
            .collect()
    }
}
