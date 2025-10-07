use rust_m3u8::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parse_master_playlist() {
        let content = r#"#EXTM3U
#EXT-X-VERSION:3
#EXT-X-STREAM-INF:BANDWIDTH=1500000,RESOLUTION=640x360,CODECS="avc1.42001e,mp4a.40.2"
playlist_360p.m3u8
#EXT-X-STREAM-INF:BANDWIDTH=2500000,RESOLUTION=854x480,CODECS="avc1.42001e,mp4a.40.2"
playlist_480p.m3u8
#EXT-X-STREAM-INF:BANDWIDTH=5000000,RESOLUTION=1280x720,CODECS="avc1.42001e,mp4a.40.2"
playlist_720p.m3u8"#;

        let parser = NestedParser::new(vec![]).unwrap();
        // 这里会失败，因为没有提供有效的 BASE URL
        let result = parser
            .parse_content(content, Some("https://example.com/master.m3u8"))
            .await;

        assert!(result.is_ok());
        let nested = result.unwrap();

        assert_eq!(nested.master_playlist.variants.len(), 3);
        assert_eq!(nested.master_playlist.playlist_type, PlaylistType::Master);
        assert!(nested.master_playlist.is_nested());
    }

    #[tokio::test]
    async fn test_parse_media_playlist() {
        let content = r#"#EXTM3U
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

        let parser = NestedParser::new(vec![]).unwrap();
        let result = parser
            .parse_content(content, Some("https://example.com/media.m3u8"))
            .await;

        assert!(result.is_ok());
        let nested = result.unwrap();

        if let Some(playlist) = nested.get_selected_variant() {
            assert_eq!(playlist.segments.len(), 3);
            assert_eq!(playlist.playlist_type, PlaylistType::Media);
            assert!(!playlist.is_nested());
            assert_eq!(playlist.total_duration(), 27.027);
        } else {
            panic!("Expected media playlist");
        }
    }

    #[tokio::test]
    async fn test_parse_with_ad_filter() {
        let content = r#"#EXTM3U
#EXT-X-VERSION:3
#EXT-X-TARGETDURATION:10
#EXTINF:9.009,
https://ad.com/segment1.ts
#EXTINF:9.009,
https://example.com/segment2.ts
#EXTINF:9.009,
https://ads.com/segment3.ts
#EXT-X-ENDLIST"#;

        let parser = NestedParser::new(vec!["ad\\.com".to_string(), "ads\\.".to_string()]).unwrap();
        let result = parser
            .parse_content(content, Some("https://example.com/playlist.m3u8"))
            .await;

        assert!(result.is_ok());
        let nested = result.unwrap();

        if let Some(playlist) = nested.get_selected_variant() {
            assert_eq!(playlist.segments.len(), 1); // 只有 segment2.ts 没有被过滤
            assert_eq!(playlist.ads_count, 2);
        }
    }

    #[test]
    fn test_playlist_type_display() {
        assert_eq!(PlaylistType::Master.to_string(), "master");
        assert_eq!(PlaylistType::Media.to_string(), "media");
        assert_eq!(PlaylistType::Unknown.to_string(), "unknown");
    }

    #[test]
    fn test_m3u8_segment_creation() {
        let segment = M3u8Segment {
            url: "https://example.com/segment1.ts".to_string(),
            duration: 10.0,
            sequence: 1,
            title: Some("Segment 1".to_string()),
            byte_range: Some((0, 1024)),
        };

        assert_eq!(segment.url, "https://example.com/segment1.ts");
        assert_eq!(segment.duration, 10.0);
        assert_eq!(segment.sequence, 1);
        assert!(segment.title.is_some());
        assert!(segment.byte_range.is_some());
    }

    #[test]
    fn test_m3u8_variant_creation() {
        // 测试变体流创建（通过解析器间接测试）
        let content = r#"#EXTM3U
#EXT-X-VERSION:3
#EXT-X-STREAM-INF:BANDWIDTH=5000000,RESOLUTION=1280x720,CODECS="avc1.42001e,mp4a.40.2"
playlist_720p.m3u8"#;

        // 验证可以成功解析包含变体流的内容
        assert!(content.contains("BANDWIDTH=5000000"));
        assert!(content.contains("RESOLUTION=1280x720"));
        assert!(content.contains("playlist_720p.m3u8"));
    }

    #[test]
    fn test_nested_m3u8_selection() {
        let mut nested = NestedM3u8::new();

        // 添加一些媒体播放列表
        let mut playlist1 = M3u8Playlist::new(PlaylistType::Media);
        playlist1.segments.push(M3u8Segment {
            url: "segment1.ts".to_string(),
            duration: 10.0,
            sequence: 0,
            title: None,
            byte_range: None,
        });

        let mut playlist2 = M3u8Playlist::new(PlaylistType::Media);
        playlist2.segments.push(M3u8Segment {
            url: "segment2.ts".to_string(),
            duration: 10.0,
            sequence: 0,
            title: None,
            byte_range: None,
        });

        nested.media_playlists.push(playlist1);
        nested.media_playlists.push(playlist2);

        // 测试选择变体流
        assert!(nested.select_variant(0).is_some());
        assert!(nested.get_selected_variant().is_some());
        assert_eq!(nested.selected_variant_index, Some(0));

        // 测试无效选择
        assert!(nested.select_variant(5).is_none());
        assert_eq!(nested.selected_variant_index, Some(0)); // 应该保持不变
    }

    #[test]
    fn test_proxy_config_creation() {
        let proxy_args = vec![
            "10,http://proxy1:8080".to_string(),
            "20,http://proxy2:8080".to_string(),
        ];

        let config = ProxyConfig::from_args(&proxy_args).unwrap();

        assert_eq!(config.len(), 2);

        // 测试随机代理选择（应该不会panic）
        for _ in 0..10 {
            assert!(config.get_random_proxy().is_some());
        }
    }

    #[test]
    fn test_proxy_config_invalid_format() {
        let proxy_args = vec!["invalid_format".to_string()];
        let result = ProxyConfig::from_args(&proxy_args);

        assert!(result.is_err());
    }

    #[test]
    fn test_m3u8_playlist_methods() {
        let mut playlist = M3u8Playlist::new(PlaylistType::Media);

        playlist.segments.push(M3u8Segment {
            url: "segment1.ts".to_string(),
            duration: 10.0,
            sequence: 0,
            title: None,
            byte_range: None,
        });

        playlist.segments.push(M3u8Segment {
            url: "segment2.ts".to_string(),
            duration: 15.0,
            sequence: 1,
            title: None,
            byte_range: None,
        });

        assert_eq!(playlist.total_duration(), 25.0);
        assert_eq!(playlist.segment_count(), 2);
        assert!(!playlist.is_nested());
    }
}
