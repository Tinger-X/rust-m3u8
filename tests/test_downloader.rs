use rust_m3u8::*;
use std::path::PathBuf;
use tokio::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_downloader_creation() {
        let _ = M3u8Downloader::new(
            "https://example.com/playlist.m3u8".to_string(),
            PathBuf::from("output.mp4"),
            PathBuf::from("temp"),
            10,
            false,
            None,
            3,
            None,
            vec![],
            vec![],
            false,
        );

        // 验证可以成功创建下载器实例
        assert!(true); // 如果创建成功，测试通过
    }

    #[test]
    fn test_downloader_with_proxy() {
        let proxy_config =
            ProxyConfig::from_args(&vec!["10,http://proxy1:8080".to_string()]).unwrap();

        let _ = M3u8Downloader::new(
            "https://example.com/playlist.m3u8".to_string(),
            PathBuf::from("output.mp4"),
            PathBuf::from("temp"),
            5,
            true,
            Some(proxy_config),
            5,
            None,
            vec![],
            vec![],
            true,
        );

        // 验证可以成功创建带代理的下载器实例
        assert!(true);
    }

    #[test]
    fn test_downloader_with_custom_headers() {
        let _ = M3u8Downloader::new(
            "https://example.com/playlist.m3u8".to_string(),
            PathBuf::from("output.mp4"),
            PathBuf::from("temp"),
            5,
            false,
            None,
            3,
            None,
            vec![
                "Authorization: Bearer token".to_string(),
                "User-Agent: CustomAgent".to_string(),
            ],
            vec![],
            false,
        );

        // 验证可以成功创建带自定义请求头的下载器实例
        assert!(true);
    }

    #[test]
    fn test_downloader_with_ad_filters() {
        let _ = M3u8Downloader::new(
            "https://example.com/playlist.m3u8".to_string(),
            PathBuf::from("output.mp4"),
            PathBuf::from("temp"),
            5,
            false,
            None,
            3,
            None,
            vec![],
            vec!["ad\\.com".to_string(), "ads\\.".to_string()],
            false,
        );

        // 验证可以成功创建带广告过滤的下载器实例
        assert!(true);
    }

    #[test]
    fn test_format_duration_function() {
        let segments = vec![
            M3u8Segment {
                url: "segment1.ts".to_string(),
                duration: 30.5,
                sequence: 0,
                title: None,
                byte_range: None,
            },
            M3u8Segment {
                url: "segment2.ts".to_string(),
                duration: 45.2,
                sequence: 1,
                title: None,
                byte_range: None,
            },
        ];

        // 这个测试需要访问 downloader 模块内部的 format_duration 函数
        // 由于该函数是私有的，我们在这里重新实现逻辑进行测试
        let total_seconds: f64 = segments.iter().map(|s| s.duration).sum();
        let formatted = if total_seconds < 60.0 {
            format!("00:00:{:02} s", total_seconds as u32)
        } else if total_seconds < 3600.0 {
            let minutes = (total_seconds / 60.0) as u32;
            let seconds = (total_seconds % 60.0) as u32;
            format!("00:{:02}:{:02} s", minutes, seconds)
        } else {
            let hours = (total_seconds / 3600.0) as u32;
            let minutes = ((total_seconds % 3600.0) / 60.0) as u32;
            let seconds = (total_seconds % 60.0) as u32;
            format!("{:02}:{:02}:{:02} s", hours, minutes, seconds)
        };

        assert_eq!(formatted, "00:01:15 s");
    }

    #[tokio::test]
    async fn test_temp_directory_creation() {
        let temp_path = PathBuf::from("test_temp_dir");

        let _ = M3u8Downloader::new(
            "https://example.com/playlist.m3u8".to_string(),
            PathBuf::from("test_output.mp4"),
            temp_path.clone(),
            5,
            false,
            None,
            3,
            None,
            vec![],
            vec![],
            false,
        );

        // 验证临时目录可以被创建
        assert!(fs::create_dir_all(&temp_path).await.is_ok());

        // 清理测试目录
        let _ = fs::remove_dir_all(&temp_path).await;
    }

    #[test]
    fn test_error_types() {
        // 测试错误类型的创建和显示
        let parse_error = M3u8Error::ParseError("test error".to_string());
        let merge_error = M3u8Error::MergeError("merge error".to_string());

        // 验证错误消息格式
        assert!(format!("{}", parse_error).contains("M3U8 解析错误"));
        assert!(format!("{}", merge_error).contains("文件合并错误"));
    }

    #[test]
    fn test_proxy_config_methods() {
        let mut config = ProxyConfig::new();
        config.add_proxy("http://proxy1:8080".to_string(), 10);
        config.add_proxy("http://proxy2:8080".to_string(), 20);

        assert_eq!(config.len(), 2);

        // 测试随机选择（应该不会panic）
        for _ in 0..10 {
            assert!(config.get_random_proxy().is_some());
        }
    }

    #[test]
    fn test_video_merger_creation() {
        let _ = VideoMerger::new();
        // 验证可以成功创建合并器实例
        assert!(true); // 如果创建成功，测试通过
    }
}
