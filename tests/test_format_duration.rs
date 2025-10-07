use rust_m3u8::M3u8Segment;

// 创建一个辅助函数来测试时长格式化
fn format_duration(segments: &[M3u8Segment]) -> String {
    let total_seconds = segments.iter().map(|s| s.duration).sum::<f64>();

    if total_seconds < 60.0 {
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration_seconds() {
        let segments = vec![
            M3u8Segment {
                url: "test.ts".to_string(),
                duration: 10.5,
                sequence: 0,
                byte_range: None,
                title: None,
            },
            M3u8Segment {
                url: "test2.ts".to_string(),
                duration: 20.3,
                sequence: 1,
                byte_range: None,
                title: None,
            },
        ];

        let result = format_duration(&segments);
        assert_eq!(result, "00:00:30 s");
    }

    #[test]
    fn test_format_duration_minutes() {
        let segments = vec![
            M3u8Segment {
                url: "test.ts".to_string(),
                duration: 90.5,
                sequence: 0,
                byte_range: None,
                title: None,
            },
            M3u8Segment {
                url: "test2.ts".to_string(),
                duration: 30.2,
                sequence: 1,
                byte_range: None,
                title: None,
            },
        ];

        let result = format_duration(&segments);
        assert_eq!(result, "00:02:00 s");
    }

    #[test]
    fn test_format_duration_hours() {
        let segments = vec![
            M3u8Segment {
                url: "test.ts".to_string(),
                duration: 3600.0,
                sequence: 0,
                byte_range: None,
                title: None,
            },
            M3u8Segment {
                url: "test2.ts".to_string(),
                duration: 1800.5,
                sequence: 1,
                byte_range: None,
                title: None,
            },
        ];

        let result = format_duration(&segments);
        assert_eq!(result, "01:30:00 s");
    }

    #[test]
    fn test_format_duration_hours_no_seconds() {
        let segments = vec![M3u8Segment {
            url: "test.ts".to_string(),
            duration: 7200.0, // 2小时
            sequence: 0,
            byte_range: None,
            title: None,
        }];

        let result = format_duration(&segments);
        assert_eq!(result, "02:00:00 s");
    }
}
