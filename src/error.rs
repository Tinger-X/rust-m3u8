use thiserror::Error;

#[derive(Error, Debug)]
pub enum M3u8Error {
    #[error("网络请求错误: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),

    #[error("URL 错误: {0}")]
    UrlError(#[from] url::ParseError),

    #[error("网络代理错误: {0}")]
    ProxyError(String),

    #[error("空对象: {0}")]
    EmptyError(String),

    #[error("下载错误: {0}")]
    DownloadError(String),

    #[error("文件不存在: {0}")]
    FileNotFoundError(std::path::PathBuf),

    #[error("正则表达式解析错误: {0}")]
    RegexError(#[from] regex::Error),

    #[error("M3U8 解析错误: {0}")]
    ParseError(String),
}
