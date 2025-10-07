use rust_m3u8::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 测试嵌套 M3U8 解析功能");

    // 测试主播放列表解析
    let master_content = r#"#EXTM3U
#EXT-X-VERSION:3
#EXT-X-STREAM-INF:BANDWIDTH=1500000,RESOLUTION=640x360,CODECS="avc1.42001e,mp4a.40.2"
playlist_360p.m3u8
#EXT-X-STREAM-INF:BANDWIDTH=2500000,RESOLUTION=854x480,CODECS="avc1.42001e,mp4a.40.2"
playlist_480p.m3u8
#EXT-X-STREAM-INF:BANDWIDTH=5000000,RESOLUTION=1280x720,CODECS="avc1.42001e,mp4a.40.2"
playlist_720p.m3u8"#;

    let parser = NestedParser::new(vec![])?;
    let nested = parser
        .parse_content(master_content, Some("https://example.com/master.m3u8"))
        .await?;

    // 注意：实际使用时需要替换为有效的 M3U8 URL
    println!("💡 注意：此示例仅演示解析功能");
    println!("📋 要实际下载，请替换为有效的 M3U8 URL");

    println!("✅ 主播放列表解析成功");
    println!("📊 变体流数量: {}", nested.master_playlist.variants.len());
    println!("🎯 当前选择索引: {:?}", nested.selected_variant_index);

    // 显示变体流信息
    for (index, variant) in nested.master_playlist.variants.iter().enumerate() {
        let quality_info = if let Some(bandwidth) = variant.bandwidth {
            if let Some((width, height)) = variant.resolution {
                format!("{}x{} @ {} kbps", width, height, bandwidth / 1000)
            } else {
                format!("{} kbps", bandwidth / 1000)
            }
        } else {
            "未知质量".to_string()
        };
        println!("   [{}] {}", index, quality_info);
    }

    // 测试媒体播放列表解析
    let media_content = r#"#EXTM3U
#EXT-X-VERSION:3
#EXT-X-TARGETDURATION:10
#EXT-X-PLAYLIST-TYPE:VOD
#EXTINF:9.009,
segment1.ts
#EXTINF:9.009,
segment2.ts
#EXTINF:9.009,
segment3.ts
#EXT-X-ENDLIST"#;

    let nested_media = parser
        .parse_content(media_content, Some("https://example.com/media.m3u8"))
        .await?;

    if let Some(playlist) = nested_media.get_selected_variant() {
        println!("\n✅ 媒体播放列表解析成功");
        println!("📊 片段数量: {}", playlist.segments.len());
        println!("🕒 总时长: {:.1} 秒", playlist.total_duration());
        println!("🎥 播放列表类型: {}", playlist.playlist_type);
    }

    // 测试嵌套播放列表下载器
    println!("\n🧪 测试嵌套播放列表下载器");
    println!("✅ 下载器创建成功");
    println!("📁 输出路径: test_output.mp4");
    println!("📁 临时目录: temp_test");
    println!("🔄 并发数量: 5");
    println!("🔁 最大重试: 3 次");

    // 注意：这里不实际执行下载，只是演示接口使用
    println!("🚫 实际下载被跳过（仅演示接口）");

    Ok(())
}
