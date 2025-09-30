use crate::utils::errors::Result;

/// TS片段信息
#[derive(Debug, Clone)]
pub struct Segment {
    pub url: String,
    pub is_ad: bool,
    pub data: Vec<u8>,
    pub size: Option<(u32, u32)>, // (height, width)
}

impl Segment {
    /// 创建新的视频片段
    pub fn new(url: String) -> Self {
        Self {
            url,
            data: Vec::new(),
            is_ad: false,
            size: None,
        }
    }

    /// TODO
    pub async fn download(&mut self, client: &Client) -> Result<()> {
        let resp = client.get(&self.url).send().await?;
        self.data = resp.bytes().await?;
        Ok(())
    }
}
