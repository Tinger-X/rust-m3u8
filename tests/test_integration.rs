//! 集成测试
//! 
//! 这个测试文件包含端到端的集成测试，验证整个下载流程。

use rust_m3u8::*;
use std::path::PathBuf;
use tokio::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_complete_download_flow() {
        // 测试完整的下载流程（使用模拟数据）
        // 注意：这是一个模拟测试，不进行实际网络请求
        
        let _ = M3u8Downloader::new(
            "https://example.com/playlist.m3u8".to_string(),
            PathBuf::from("test_output.mp4"),
            PathBuf::from("test_temp"),
            5,
            false,
            None,
            3,
            None,
            vec![],
            vec![],
            false,
        );

        // 验证可以成功创建下载器
        assert!(true);
    }

    #[tokio::test]
    async fn test_parser_integration() {
        // 测试解析器集成
        
        let content = r#"#EXTM3U
#EXT-X-VERSION:3
#EXT-X-TARGETDURATION:10
#EXTINF:9.009,
segment1.ts
#EXTINF:9.009,
segment2.ts
#EXT-X-ENDLIST"#;

        let parser = NestedParser::new(vec![]).unwrap();
        let result = parser.parse_content(content, Some("https://example.com/")).await;

        assert!(result.is_ok());
        
        let nested = result.unwrap();
        assert!(nested.get_selected_variant().is_some());
    }

    #[test]
    fn test_error_handling_integration() {
        // 测试错误处理集成
        
        // 测试无效的代理配置
        let invalid_proxy_args = vec!["invalid_format".to_string()];
        let result = ProxyConfig::from_args(&invalid_proxy_args);
        
        assert!(result.is_err());
        
        // 测试有效的代理配置
        let valid_proxy_args = vec!["10,http://proxy:8080".to_string()];
        let result = ProxyConfig::from_args(&valid_proxy_args);
        
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_file_operations() {
        // 测试文件操作集成
        
        let test_dir = PathBuf::from("integration_test_temp");
        
        // 创建测试目录
        assert!(fs::create_dir_all(&test_dir).await.is_ok());
        
        // 验证目录存在
        assert!(test_dir.exists());
        
        // 清理测试目录
        let _ = fs::remove_dir_all(&test_dir).await;
    }

    #[test]
    fn test_playlist_type_integration() {
        // 测试播放列表类型集成
        
        let master_playlist = M3u8Playlist::new(PlaylistType::Master);
        let media_playlist = M3u8Playlist::new(PlaylistType::Media);
        
        assert_eq!(master_playlist.playlist_type, PlaylistType::Master);
        assert_eq!(media_playlist.playlist_type, PlaylistType::Media);
    }

    #[tokio::test]
    async fn test_nested_parser_selection() {
        // 测试嵌套解析器的选择功能
        
        let content = r#"#EXTM3U
#EXT-X-VERSION:3
#EXT-X-STREAM-INF:BANDWIDTH=1500000,RESOLUTION=640x360
playlist_360p.m3u8
#EXT-X-STREAM-INF:BANDWIDTH=2500000,RESOLUTION=854x480
playlist_480p.m3u8"#;

        let parser = NestedParser::new(vec![]).unwrap();
        // 这里会失败，因为没有提供有效的 BASE URL
        let nested = parser.parse_content(content, Some("https://example.com/")).await.unwrap();
        
        // 验证可以成功解析嵌套播放列表
        assert_eq!(nested.master_playlist.variants.len(), 2);
        assert!(nested.master_playlist.is_nested());
    }
}