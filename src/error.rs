use thiserror::Error;

#[derive(Error, Debug)]
pub enum M3u8Error {
    #[error("网络请求错误: {0}")]
    Network(#[from] reqwest::Error),

    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("URL 解析错误: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("M3U8 解析错误: {0}")]
    ParseError(String),

    #[error("文件合并错误: {0}")]
    MergeError(String),

    #[error("其他错误: {0}")]
    Other(#[from] anyhow::Error),
}
