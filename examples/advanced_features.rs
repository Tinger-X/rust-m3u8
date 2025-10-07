//! 高级功能使用示例
//!
//! 这个示例展示了 rust-m3u8 库的高级功能，包括代理、广告过滤、自定义请求头等。

use rust_m3u8::*;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 开始高级功能示例");

    // 配置代理（权重代理）
    let proxy_config = ProxyConfig::from_args(&vec![
        "10,http://proxy1.example.com:8080".to_string(),
        "15,http://proxy2.example.com:8080".to_string(),
        "20,http://proxy3.example.com:8080".to_string(),
    ])?;

    println!("🌐 代理配置: 3 个代理服务器，权重分别为 10, 15, 20");

    // 创建下载器实例（包含所有高级功能）
    let _ = M3u8Downloader::new(
        "https://example.com/master.m3u8".to_string(), // 嵌套 M3U8 URL
        PathBuf::from("high_quality_video.mp4"),       // 输出文件路径
        PathBuf::from("temp_advanced"),                // 临时目录
        12,                                            // 并发下载数量
        true,                                          // 保留临时文件（用于调试）
        Some(proxy_config),                            // 使用代理
        5,                                             // 最大重试次数
        None,                                          // 无基础 URL
        vec![
            "Authorization: Bearer your_token_here".to_string(),
            "User-Agent: CustomVideoDownloader/1.0".to_string(),
            "Referer: https://example.com".to_string(),
        ], // 自定义请求头
        vec![
            "ad\\.com".to_string(), // 广告过滤规则
            "ads\\.".to_string(),
            "tracking\\.".to_string(),
        ], // 广告过滤
        true,                                          // 使用 FFmpeg 合并
    );

    println!("📋 高级下载器配置完成");
    println!("🎯 功能特性:");
    println!("   • 嵌套 M3U8 支持");
    println!("   • 多代理负载均衡");
    println!("   • 自定义 HTTP 请求头");
    println!("   • 广告片段自动过滤");
    println!("   • FFmpeg 高质量合并");
    println!("   • 临时文件保留（调试模式）");

    println!("🎯 高级功能配置完成");
    println!("💡 下载器已创建，包含所有高级功能");

    // 注意：在实际使用中取消注释以下行来执行下载
    // println!("\n📥 开始下载...");
    // downloader.download().await?;

    println!("\n✅ 高级功能示例演示完成");
    println!("💡 要实际运行下载，请取消注释 downloader.download().await? 行");

    Ok(())
}

// 辅助函数：演示如何手动解析 M3U8 内容
async fn _demonstrate_manual_parsing() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 手动解析示例");

    let content = r#"#EXTM3U
#EXT-X-VERSION:3
#EXT-X-TARGETDURATION:10
#EXTINF:9.009,
segment1.ts
#EXTINF:9.009,
segment2.ts
#EXT-X-ENDLIST"#;

    let parser = NestedParser::new(vec![])?;
    let nested = parser
        .parse_content(content, Some("https://example.com/"))
        .await?;

    if let Some(playlist) = nested.get_selected_variant() {
        println!("📊 解析结果:");
        println!("   • 片段数量: {}", playlist.segments.len());
        println!("   • 总时长: {:.1} 秒", playlist.total_duration());
        println!("   • 播放列表类型: {}", playlist.playlist_type);
    }

    Ok(())
}
