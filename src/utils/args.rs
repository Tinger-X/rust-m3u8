use clap::Parser;

/// M3U8下载器命令行参数
#[derive(Parser, Debug)]
#[command(name = "rust-m3u8", author, version, about = "M3U8视频下载器")]
pub struct Cli {
    /// M3U8源文件地址
    #[arg(
        required = true,
        help = "M3U8源文件地址，支持http(s)以及本地文件"
    )]
    pub src: String,

    /// 输出文件名（不含扩展名）
    #[arg(
        short = 'd',
        long = "dest",
        help = "下载完成后输出的视频文件名（不含扩展，默认当前时间）"
    )]
    pub dest: Option<String>,

    /// 配置文件路径
    #[arg(short = 'c', long = "config", help = "配置文件路径，不指定则使用默认配置")]
    pub config: Option<String>,

    /// 基础url
    #[arg(short = 'b', long = "base-url", help = "基础url，用于拼接ts片段uri")]
    pub base_url: Option<String>,

    /// HTTP请求头，格式: "Key:Value"
    #[arg(short = 'H', long = "header", action = clap::ArgAction::Append, help = "添加或覆盖HTTP请求头信息，格式: Key:Value")]
    pub headers: Vec<String>,
}
