use crate::utils::errors::{Result, M3u8Error};
use bytes::Bytes;
use reqwest::blocking::Client;

/// TS片段信息
#[derive(Debug, Clone)]
pub struct Segment {
    pub url: String,
    pub is_ad: bool,
    pub data: Bytes,
    pub is_ok: bool,
    pub size: Option<(u32, u32)>, // (height, width)
}

impl Segment {
    /// 创建新的视频片段
    pub fn new(url: String) -> Self {
        Self {
            url,
            data: Bytes::new(),
            is_ad: false,
            size: None,
            is_ok: false,
        }
    }

    pub fn download(&mut self, client: &Client) -> Result<()> {
        match client.get(&self.url).send() {
            Ok(response) => {
                match response.bytes() {
                    Ok(b) => {
                        if b.is_empty() {
                            return Err(M3u8Error::EmptyContent("内容为空".to_string()));
                        }
                        self.data = b;
                        self.is_ok = true;
                        return Ok(());
                    }
                    Err(e) => {
                        return Err(M3u8Error::DownloadFailed(e.to_string()));
                    }
                }
            }
            Err(e) => {
                return Err(M3u8Error::DownloadFailed(e.to_string()));
            }
        }
    }
}
