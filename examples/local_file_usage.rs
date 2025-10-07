//! 本地文件 M3U8 下载示例
//! 
//! 这个示例展示了如何使用 rust-m3u8 库处理本地 M3U8 文件。

use rust_m3u8::*;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📁 开始本地文件 M3U8 下载示例");

    // 创建下载器实例（使用本地文件）
    let _ = M3u8Downloader::new(
        "local_playlist.m3u8".to_string(),              // 本地 M3U8 文件路径
        PathBuf::from("local_output.mp4"),             // 输出文件路径
        PathBuf::from("temp_local"),                   // 临时目录
        5,                                              // 并发下载数量
        false,                                          // 不保留临时文件
        None,                                           // 不使用代理
        3,                                              // 最大重试次数
        Some("https://example.com/base/".to_string()), // 基础 URL（用于相对路径）
        vec![],                                         // 无自定义请求头
        vec![],                                         // 无广告过滤
        false,                                          // 不使用 FFmpeg
    );

    println!("📋 本地文件下载器配置完成");
    println!("📄 输入文件: local_playlist.m3u8");
    println!("💾 输出文件: local_output.mp4");
    println!("📁 临时目录: temp_local");
    println!("🌐 基础 URL: https://example.com/base/");
    println!("🔄 并发数量: 5");
    println!("🔁 最大重试: 3 次");

    // 注意：在实际使用中取消注释以下行来执行下载
    // downloader.download().await?;

    println!("✅ 本地文件示例演示完成");
    println!("💡 要实际运行下载，请取消注释 downloader.download().await? 行");
    println!("💡 确保 local_playlist.m3u8 文件存在且包含有效的 M3U8 内容");

    Ok(())
}