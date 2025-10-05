use chrono::Utc;
use md5::{Digest, Md5};

use crate::{trace_fmt, debug_fmt};
use super::errors::Result;

pub struct Funcs;

impl Funcs {
    /// 确保输出文件夹存在，同时将输出文件后缀置为.ts
    pub fn ensure_filepath(filename: &str) -> Result<String> {
        trace_fmt!("检查输出文件");
        let filepath = if !filename.ends_with(".ts") {
            format!("{}.ts", filename)
        } else {
            filename.to_string()
        };
        debug_fmt!("输出文件: {}", filepath);

        let parent = std::path::Path::new(&filepath).parent().unwrap();
        if !parent.exists() {
            debug_fmt!("创建输出目录: {}", parent.display());
            std::fs::create_dir_all(parent)?;
        }

        Ok(filepath)
    }

    /// 生成文件名
    pub fn generate_filename() -> String {
        trace_fmt!("生成默认时间戳文件名");
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
