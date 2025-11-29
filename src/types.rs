use serde::{Deserialize, Serialize};
use std::fmt;

// M3U8 播放列表类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlaylistType {
    // 主播放列表，包含多个变体
    Master,
    // 媒体播放列表，包含具体片段
    Media,
    // 未知类型
    Unknown,
}

impl fmt::Display for PlaylistType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlaylistType::Master => write!(f, "master"),
            PlaylistType::Media => write!(f, "media"),
            PlaylistType::Unknown => write!(f, "unknown"),
        }
    }
}

// M3U8 片段信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct M3u8Segment {
    pub url: String,
    pub duration: f64,
    pub sequence: usize,
    pub title: Option<String>,
    pub byte_range: Option<(usize, usize)>,
}

// M3U8 变体流信息（用于主播放列表）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct M3u8Variant {
    pub url: String,
    pub bandwidth: Option<u32>,
    pub resolution: Option<(u32, u32)>,
    pub codecs: Option<String>,
    pub audio: Option<String>,
    pub video: Option<String>,
    pub subtitles: Option<String>,
}

// M3U8 播放列表信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct M3u8Playlist {
    pub segments: Vec<M3u8Segment>,
    pub variants: Vec<M3u8Variant>,
    pub playlist_type: PlaylistType,
    pub target_duration: f64,
    pub version: u32,
    pub is_live: bool,
    pub ads_count: usize,
    pub media_sequence: usize,
    pub discontinuity_sequence: usize,
}

// 嵌套播放列表结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedM3u8 {
    pub master_playlist: M3u8Playlist,
    pub media_playlists: Vec<M3u8Playlist>,
    pub selected_variant_index: Option<usize>,
}

impl M3u8Playlist {
    pub fn new(playlist_type: PlaylistType) -> Self {
        Self {
            segments: Vec::new(),
            variants: Vec::new(),
            playlist_type,
            target_duration: 0.0,
            version: 1,
            is_live: true,
            ads_count: 0,
            media_sequence: 0,
            discontinuity_sequence: 0,
        }
    }

    // 获取总时长
    pub fn total_duration(&self) -> f64 {
        self.segments.iter().map(|s| s.duration).sum()
    }

    // 获取片段数量
    pub fn segment_count(&self) -> usize {
        self.segments.len()
    }

    // 检查是否为嵌套播放列表
    pub fn is_nested(&self) -> bool {
        !self.variants.is_empty() && self.playlist_type == PlaylistType::Master
    }
}

impl NestedM3u8 {
    pub fn new() -> Self {
        Self {
            master_playlist: M3u8Playlist::new(PlaylistType::Master),
            media_playlists: Vec::new(),
            selected_variant_index: None,
        }
    }

    // 选择特定的变体流
    pub fn select_variant(&mut self, index: usize) -> Option<&M3u8Playlist> {
        if index < self.media_playlists.len() {
            self.selected_variant_index = Some(index);
            Some(&self.media_playlists[index])
        } else {
            None
        }
    }

    // 获取当前选中的变体流
    pub fn get_selected_variant(&self) -> Option<&M3u8Playlist> {
        self.selected_variant_index
            .and_then(|idx| self.media_playlists.get(idx))
    }
}

impl Default for NestedM3u8 {
    fn default() -> Self {
        Self::new()
    }
}
