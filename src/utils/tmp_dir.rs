use md5::{Digest, Md5};
use std::env;
use std::fs;
use std::path::PathBuf;

use super::errors::Result;

/// 临时文件夹管理器
#[derive(Debug, Clone)]
pub struct TempDir {
    path: PathBuf,
}

fn generate_temp_dir(content: &str) -> PathBuf {
    let mut temp_dir = env::temp_dir();
    let mut hasher = Md5::new();
    hasher.update(content.as_bytes());
    let filename = format!("rust_m3u8_{:x}", hasher.finalize());
    temp_dir.push(filename);
    temp_dir
}

impl TempDir {
    pub fn new() -> Self {
        Self {
            path: PathBuf::new(),
        }
    }

    pub fn init(&mut self, content: &str) -> Result<()> {
        let temp_dir = generate_temp_dir(&content);
        if !temp_dir.exists() {
            fs::create_dir_all(&temp_dir)?;
        }
        self.path = temp_dir;
        Ok(())
    }

    pub fn get_file_path(&self, index: &u32) -> PathBuf {
        self.path.join(format!("{:08}.ts", index))
    }

    pub fn file_exists(&self, index: &u32) -> bool {
        self.get_file_path(index).exists()
    }

    /// 写入文件
    pub async fn write(&self, index: &u32, data: &[u8]) -> Result<()> {
        let path = self.path.join(format!("{:08}.ts", index));
        async_std::fs::write(path, data).await?;
        Ok(())
    }

    /// 加载文件内容
    pub async fn load(&self, index: &u32, data: &mut Vec<u8>) -> Result<bool> {
        if self.file_exists(index) {
            let path = self.path.join(format!("{:08}.ts", index));
            *data = async_std::fs::read(path).await?;
            return Ok(true);
        }
        Ok(false)
    }

    /// 完整下载后清理临时文件夹
    pub fn cleanup(&mut self) -> Result<()> {
        if self.path.exists() {
            fs::remove_dir_all(&self.path)?;
        }
        Ok(())
    }
}
