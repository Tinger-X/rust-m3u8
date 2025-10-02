use bytes::Bytes;
use chrono::Utc;
use md5::{Digest, Md5};

use crate::warn_fmt;

pub struct Funcs;

impl Funcs {
    /// 确保文件名有正确的扩展名，扩展名只能取：mp4, avi, mkv, flv, ts
    pub fn ensure_extension(filename: &str, extension: &str) -> String {
        match extension {
            "mp4" | "avi" | "mkv" | "flv" | "ts" => {
                if filename.ends_with(format!(".{}", extension).as_str()) {
                    filename.to_string()
                } else {
                    format!("{}.{}", filename, extension)
                }
            }
            _ => {
                warn_fmt!(
                    "不支持的扩展名 '{}', 仅支持 mp4(默认，当前仅支持), avi, mkv, flv, ts",
                    extension
                );
                format!("{}.{}", filename, "mp4")
            }
        }
    }

    /// 生成默认文件名
    pub fn generate_default_filename() -> String {
        Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string()
    }

    pub fn is_online_resource(src: &str) -> bool {
        return src.starts_with("http://") || src.starts_with("https://");
    }

    /// 计算字符串的MD5哈希值
    pub fn content_digest(s: &str) -> String {
        let mut hasher = Md5::new();
        hasher.update(s.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn decode_video_size(data: &Bytes) -> Option<(u32, u32)> {
        // 将Bytes转换为Cursor以便进行读取操作
        None
    }
}
