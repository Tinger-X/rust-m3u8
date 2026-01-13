use crate::error::M3u8Error;
use std::path::PathBuf;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct VideoMerger {
    temp_dir: PathBuf,
    output_path: PathBuf,
    segments: usize,
}

impl VideoMerger {
    pub async fn new(
        temp_dir: &PathBuf,
        output_path: &PathBuf,
        segments: usize,
    ) -> Result<Self, M3u8Error> {
        // 创建file_list.txt
        let file_list_path = temp_dir.join("file_list.txt");
        let mut file_list_content = String::new();

        for index in 0..segments {
            let segment_path = temp_dir.join(format!("seg{:06}.ts", index));
            if !segment_path.exists() {
                return Err(M3u8Error::MergeError(format!(
                    "片段文件不存在: {:?}",
                    segment_path
                )));
            }

            // 获取绝对路径并转换为 FFmpeg 兼容格式
            let absolute_path = segment_path
                .canonicalize()
                .map_err(|e| M3u8Error::MergeError(format!("无法获取绝对路径: {}", e)))?;

            // 在 Windows 上将反斜杠转换为正斜杠，FFmpeg 更好地支持正斜杠
            let path_str = absolute_path.to_string_lossy().replace('\\', "/");
            file_list_content.push_str(&format!("file '{}'\n", path_str));
        }
        // 写入文件列表
        fs::write(&file_list_path, file_list_content).await?;

        // 确保output_path的文件夹存在，不存在则创建
        if let Some(parent) = output_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).await?;
            }
        }

        Ok(Self {
            temp_dir: temp_dir.clone(),
            output_path: output_path.clone(),
            segments: segments,
        })
    }

    pub async fn merge_with_rust(&self) -> Result<(), M3u8Error> {
        let mut output_file = fs::File::create(&self.output_path).await?;

        for index in 0..self.segments {
            let segment_path = self.temp_dir.join(format!("seg{:06}.ts", index));
            let mut segment_file = fs::File::open(&segment_path).await?;
            let mut buffer = Vec::new();
            segment_file.read_to_end(&mut buffer).await?;

            output_file.write_all(&buffer).await?;
        }

        output_file.flush().await?;
        println!(
            "✅ 成功合并 {} 个片段到 {:?}",
            self.segments, self.output_path
        );

        Ok(())
    }

    pub async fn merge_with_ffmpeg(&self) -> Result<(), M3u8Error> {
        // 使用 ffmpeg 合并
        let file_list_path = self.temp_dir.join("file_list.txt");
        let output = std::process::Command::new("ffmpeg")
            .args(&[
                "-f",
                "concat",
                "-safe",
                "0",
                "-i",
                file_list_path.to_str().unwrap(),
                "-c",
                "copy",
                "-y",
                self.output_path.to_str().unwrap(),
            ])
            .output();

        match output {
            Ok(output) => {
                if !output.status.success() {
                    let error_msg = String::from_utf8_lossy(&output.stderr);
                    println!(
                        "⚠️  FFmpeg 合并失败，回退到简单合并模式。 错误信息: {}",
                        error_msg
                    );

                    // 回退到简单合并
                    return self.merge_with_rust().await;
                }
                println!(
                    "✅ 成功合并 {} 个片段到 {:?}",
                    self.segments, self.output_path
                );
            }
            Err(e) => {
                println!("⚠️  FFmpeg 不可用，使用简单合并: {}", e);
                return self.merge_with_rust().await;
            }
        }

        Ok(())
    }
}
