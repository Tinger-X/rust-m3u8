use clap::Parser;
use rust_m3u8::{M3u8Downloader, M3u8Error, ProxyConfig};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rust-m3u8")]
#[command(about = "一个用 Rust 编写的 M3U8 下载器", version)]
struct Args {
    /// M3U8 播放列表的 URL 或本地文件路径
    url: String,

    /// 输出文件名
    #[arg(short, long, default_value = "output")]
    output: String,

    /// 并发下载数量
    #[arg(short, long, default_value = "20")]
    concurrent: usize,

    /// 临时文件目录
    #[arg(short, long, default_value = "temp")]
    temp_dir: String,

    /// 最大重试次数
    #[arg(short, long, default_value = "3")]
    retry: usize,

    /// 使用 简单方式 合并视频片段，默认使用 ffmpeg
    #[arg(long, long)]
    simple: bool,

    /// 代理配置，格式: "weight,proxy_url"，可多次指定
    #[arg(short, long, action = clap::ArgAction::Append)]
    proxy: Vec<String>,

    /// 基础 URL（当使用本地 M3U8 文件且包含相对路径时必需）
    #[arg(short, long)]
    base: Option<String>,

    /// 自定义请求头，格式: "Name: Value"，可多次指定
    #[arg(short = 'H', long, action = clap::ArgAction::Append)]
    header: Vec<String>,

    /// 广告过滤正则表达式，可多次指定
    #[arg(short, long, action = clap::ArgAction::Append)]
    filter: Vec<String>,

    /// 下载完成后是否保留临时文件
    #[arg(long)]
    keep_temp: bool,
}

#[tokio::main]
async fn main() -> Result<(), M3u8Error> {
    let args = Args::parse();

    // 解析代理配置
    let mut proxy_count: u32 = 0;
    let proxy_config = if !args.proxy.is_empty() {
        match ProxyConfig::from_args(&args.proxy) {
            Ok(config) => {
                proxy_count = config.len() as u32;
                Some(config)
            }
            Err(e) => {
                eprintln!("❌ 代理配置错误: {}", e);
                return Err(M3u8Error::ParseError(e));
            }
        }
    } else {
        None
    };

    println!(
        "已配置: 🌐 代理 {} 个, 🚫 广告过滤规则 {} 条\n📁 输出文件: {}.mp4, 🔄 并发数量: {}, 🔁 最大重试: {} 次",
        proxy_count,
        args.filter.len(),
        args.output, args.concurrent, args.retry
    );

    let downloader = M3u8Downloader::new(
        args.url,
        PathBuf::from(args.output).with_extension("mp4"),
        PathBuf::from(args.temp_dir),
        args.concurrent,
        args.keep_temp,
        proxy_config,
        args.retry,
        args.base,
        args.header,
        args.filter,
        args.simple,
    );

    downloader.download().await?;
    Ok(())
}
