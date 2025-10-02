use std::env;
use std::fs;
use std::path::PathBuf;
use bytes::Bytes;

use super::common::Funcs;
use super::errors::Result;
use crate::{error_fmt, warn_fmt};

/// 临时文件夹管理器
#[derive(Debug, Clone)]
pub struct TempDir {
    path: PathBuf,
}

fn generate_temp_dir(content: &str) -> PathBuf {
    let mut temp_dir = env::temp_dir();
    let filename = format!("rust_m3u8_{}", Funcs::content_digest(content));
    temp_dir.push(filename);
    temp_dir
}

impl TempDir {
    fn get_file_path(&self, index: &usize) -> PathBuf {
        self.path.clone().join(format!("{:08}.ts", index))
    }

    fn file_exists(&self, index: &usize) -> bool {
        self.get_file_path(index).exists()
    }
}

impl TempDir {
    pub fn new() -> Self {
        Self {
            path: PathBuf::new(),
        }
    }

    pub fn init(&mut self, content: &str) {
        let temp_dir = generate_temp_dir(&content);
        if !temp_dir.exists() {
            match fs::create_dir_all(&temp_dir) {
                Ok(_) => {}
                Err(e) => {
                    error_fmt!("创建临时文件夹 {} 失败: {}", temp_dir.display(), e);
                }
            }
        }
        self.path = temp_dir;
    }

    /// 写入文件
    pub fn write(&self, index: &usize, data: &Bytes) {
        let path = self.get_file_path(index);

        match std::fs::write(&path, data) {
            Ok(_) => {}
            Err(e) => {
                error_fmt!("写入文件 {} 失败: {}", path.display(), e);
            }
        }
    }

    /// 加载文件内容
    pub fn load(&self, index: &usize, data: &mut Bytes) -> bool {
        let path = self.get_file_path(index);
        if self.file_exists(index) {
            match std::fs::read(&path) {
                Ok(content) => {
                    if content.is_empty() {
                        warn_fmt!("片段 {} 内容为空", index);
                        return false;
                    }
                    *data = content.into();
                    true
                }
                Err(e) => {
                    warn_fmt!("读取片段 {} 失败: {}", index, e);
                    false
                }
            }
        } else {
            false
        }
    }

    /// 完整下载后清理临时文件夹
    pub fn cleanup(&mut self) -> Result<()> {
        if self.path.exists() {
            match fs::remove_dir_all(&self.path) {
                Ok(_) => {}
                Err(e) => {
                    warn_fmt!("删除临时文件夹 {} 失败: {}", self.path.display(), e);
                }
            }
        }
        Ok(())
    }
}
