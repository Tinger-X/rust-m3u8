use chrono::Utc;
use md5::{Md5, Digest};

pub struct Funcs {
    pub ensure_extension: fn(&str, &str) -> String,
    pub generate_default_filename: fn() -> String,
    pub is_online_resource: fn(&str) -> bool,
}

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
            _ => panic!("不支持的扩展名，仅支持 mp4, avi, mkv, flv, ts"),
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
}
