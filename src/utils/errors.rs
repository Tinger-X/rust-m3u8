use indicatif::style::TemplateError;
use regex::Error as RegexError;
use reqwest::Error as ReqwestError;
use std::io::Error as IoError;
use thiserror::Error;

/// 应用程序错误类型
#[derive(Error, Debug)]
pub enum M3u8Error {
    /// IO错误
    #[error("IO error: {0}")]
    Io(#[from] IoError),

    /// HTTP请求错误
    #[error("HTTP request error: {0}")]
    HttpRequest(#[from] ReqwestError),

    /// 下载失败
    #[error("Download failed: {0}")]
    DownloadFailed(String),

    /// 迭代错误
    #[error("Empty content error: {0}")]
    EmptyContent(String),

    /// 无效的HTTP头值
    #[error("Invalid HTTP header value: {0}")]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),

    /// 无效的HTTP头名称
    #[error("Invalid HTTP header name: {0}")]
    InvalidHeaderName(#[from] reqwest::header::InvalidHeaderName),

    /// 正则表达式错误
    #[error("Regex error: {0}")]
    Regex(#[from] RegexError),

    /// M3U8解析错误
    #[error("M3U8 parse error: {0}")]
    M3U8Parse(String),

    /// 进度条模板错误
    #[error("Progress bar template error: {0}")]
    TemplateError(#[from] TemplateError),

    /// 配置解析错误
    #[error("Failed to parse config file: {0}")]
    ParseError(#[from] toml::de::Error),

    /// URL解析错误
    #[error("URL join error: {0}")]
    UrlJoin(#[from] url::ParseError),

    /// URL解析错误
    #[error("URL parse error: {0}")]
    UrlParse(String),

    /// 未实现错误
    #[error("Not implemented: {0}")]
    _ToBeImplemented(String),
}

/// 结果类型别名
pub type Result<T> = std::result::Result<T, M3u8Error>;
