use crate::utils::errors::{M3u8Error, Result};
use bytes::Bytes;
use reqwest::blocking::Client;

/// TS片段信息
#[derive(Debug, Clone)]
pub struct Segment {
    pub url: String,
    pub is_ad: bool,
    pub data: Bytes,
    pub is_ok: bool,
    pub _size: Option<(u32, u32)>, // (height, width)
}

impl Segment {
    /// 创建新的视频片段
    pub fn new(url: String) -> Self {
        Self {
            url,
            data: Bytes::new(),
            is_ad: false,
            _size: None,
            is_ok: false,
        }
    }

    pub fn download(&mut self, client: &Client) -> Result<()> {
        let response = client.get(&self.url).send()?;
        if !response.status().is_success() {
            return Err(M3u8Error::DownloadFailed(format!("下载失败，状态码：{}", response.status())));
        }
        self.data = response.bytes()?;
        if self.data.is_empty() {
            return Err(M3u8Error::EmptyContent("内容为空".to_string()));
        }
        self.is_ok = true;
        Ok(())
    }

    pub fn _decode_resolution(&mut self) -> Result<()> {
        // TODO：解析TS片段的分辨率
        Err(M3u8Error::_ToBeImplemented("当前版本不支持解析TS片段的分辨率".to_string()))
    }
}
