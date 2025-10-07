//! 基础 M3U8 下载器使用示例
//! 
//! 这个示例展示了如何使用 rust-m3u8 库的基本功能来下载 M3U8 视频。

use rust_m3u8::*;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 开始基础 M3U8 下载示例");

    // 创建下载器实例
    let _ = M3u8Downloader::new(
        "https://example.com/playlist.m3u8".to_string(), // M3U8 URL
        PathBuf::from("output_video.mp4"),              // 输出文件路径
        PathBuf::from("temp_download"),                 // 临时目录
        8,                                              // 并发下载数量
        false,                                          // 不保留临时文件
        None,                                           // 不使用代理
        3,                                              // 最大重试次数
        None,                                           // 无基础 URL
        vec![],                                         // 无自定义请求头
        vec![],                                         // 无广告过滤
        false,                                          // 不使用 FFmpeg
    );

    println!("📋 下载器配置完成");
    println!("🌐 URL: https://example.com/playlist.m3u8");
    println!("💾 输出文件: output_video.mp4");
    println!("📁 临时目录: temp_download");
    println!("🔄 并发数量: 8");
    println!("🔁 最大重试: 3 次");

    // 注意：在实际使用中取消注释以下行来执行下载
    // downloader.download().await?;

    println!("✅ 示例演示完成（实际下载被注释）");
    println!("💡 要实际运行下载，请取消注释 downloader.download().await? 行");

    Ok(())
}