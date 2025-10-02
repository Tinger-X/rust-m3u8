use clap::Parser;

use crate::warn_fmt;
use super::config::AppConfig;

/// M3U8下载器命令行参数
#[derive(Parser, Debug)]
#[command(name = "rust-m3u8", author, version, about = "M3U8视频下载器")]
pub struct Cli {
    /// M3U8源文件地址
    #[arg(short = 's', long = "src", required = true, help = "M3U8源文件地址，支持http(s)以及本地文件")]
    pub src: String,

    /// 输出文件名（不含扩展名）
    #[arg(short = 'd', long = "dest", help = "下载完成后输出的视频文件名（不含扩展）")]
    pub dest: Option<String>,

    /// 输出视频格式
    #[arg(short = 'e', long = "ext", default_value = "mp4", help = "输出视频的后缀及保存格式，支持: mp4, avi, mkv, flv, ts")]
    pub ext: String,

    /// 配置文件路径
    #[arg(short = 'c', long = "config", help = "配置文件路径")]
    pub config: Option<String>,

    /// HTTP请求头，格式: "Key:Value"
    #[arg(short = 'H', long = "header", action = clap::ArgAction::Append, help = "添加或覆盖HTTP请求头信息，格式: Key:Value")]
    pub headers: Vec<String>,
}

impl Cli {
    pub fn update_config_headers(&self, config: &mut AppConfig) {
        if self.headers.is_empty() {
            return;
        }
        for header in &self.headers {
            if let Some((key, value)) = header.split_once(':') {
                config.headers.insert(key.trim().to_string(), value.trim().to_string());
            } else {
                warn_fmt!("已忽略无法解析的请求头：{}", header);
            }
        }
    }
}