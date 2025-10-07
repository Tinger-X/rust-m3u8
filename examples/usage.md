# M3U8 下载器使用指南

## 快速开始

### 安装
```bash
# 从源码安装
cargo install --path .

# 或直接运行
cargo run -- [参数]
```

### 基本用法
```bash
# 下载单个 M3U8 视频
rust-m3u8 https://example.com/playlist.m3u8

# 下载嵌套播放列表（自动选择最佳质量）
rust-m3u8 https://example.com/master.m3u8

# 指定输出文件名
rust-m3u8 https://example.com/playlist.m3u8 -o video

# 设置并发数量
rust-m3u8 https://example.com/playlist.m3u8 -c 10
```

## 命令行参数详解

### 必需参数
- `URL`: M3U8 播放列表的 URL 或本地文件路径

### 可选参数
- `-o, --output <OUTPUT>`: 输出文件名（默认: "output"）
- `-c, --concurrent <CONCURRENT>`: 并发下载数量（默认: 20）
- `-t, --temp-dir <TEMP_DIR>`: 临时文件目录（默认: "temp"）
- `-r, --retry <RETRY>`: 最大重试次数（默认: 3）
- `-p, --proxy <PROXY>`: 代理配置，格式: "weight,proxy_url"（可多次指定）
- `-b, --base <BASE>`: 基础 URL（当使用本地 M3U8 文件且包含相对路径时必需）
- `-H, --header <HEADER>`: 自定义请求头，格式: "Name: Value"（可多次指定）
- `-f, --filter <FILTER>`: 广告过滤正则表达式（可多次指定）
- `--keep-temp`: 下载完成后保留临时文件
- `--use-ffmpeg`: 使用系统 FFmpeg 合并视频片段

## 示例场景

### 1. 基础下载
```bash
# 下载标准 M3U8 视频
rust-m3u8 https://example.com/playlist.m3u8 -o my_video
```

### 2. 高并发下载
```bash
# 使用 15 个并发连接下载
rust-m3u8 https://example.com/playlist.m3u8 -c 15 -o fast_download
```

### 3. 使用代理
```bash
# 使用多个代理服务器（权重负载均衡）
rust-m3u8 https://example.com/playlist.m3u8 \
  -p "10,http://proxy1:8080" \
  -p "15,http://proxy2:8080" \
  -p "20,http://proxy3:8080"
```

### 4. 广告过滤
```bash
# 过滤广告片段
rust-m3u8 https://example.com/playlist.m3u8 \
  -f "ad\\.com" \
  -f "ads\\." \
  -f "tracking\\."
```

### 5. 自定义请求头
```bash
# 添加认证和自定义 User-Agent
rust-m3u8 https://example.com/playlist.m3u8 \
  -H "Authorization: Bearer your_token" \
  -H "User-Agent: CustomDownloader/1.0" \
  -H "Referer: https://example.com"
```

### 6. 本地文件处理
```bash
# 处理本地 M3U8 文件
rust-m3u8 local_playlist.m3u8 -b "https://example.com/base/"
```

### 7. 高质量合并
```bash
# 使用 FFmpeg 进行高质量合并（需要安装 FFmpeg）
rust-m3u8 https://example.com/playlist.m3u8 --use-ffmpeg
```

### 8. 调试模式
```bash
# 保留临时文件用于调试
rust-m3u8 https://example.com/playlist.m3u8 --keep-temp -t debug_temp
```

## 高级配置示例

### 完整配置示例
```bash
rust-m3u8 \
  "https://example.com/master.m3u8" \
  --output "high_quality_video" \
  --concurrent 12 \
  --temp-dir "download_temp" \
  --retry 5 \
  --proxy "10,http://proxy1:8080" \
  --proxy "15,http://proxy2:8080" \
  --header "Authorization: Bearer token" \
  --header "User-Agent: CustomAgent/1.0" \
  --filter "ad\\." \
  --filter "tracking\\." \
  --use-ffmpeg \
  --keep-temp
```

## 程序库使用示例

### 基本使用
```rust
use rust_m3u8::*;
use std::path::PathBuf;

let downloader = M3u8Downloader::new(
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

downloader.download().await?;
```

### 高级功能
```rust
use rust_m3u8::*;
use std::path::PathBuf;

// 配置代理
let proxy_config = ProxyConfig::from_args(&vec![
    "10,http://proxy1:8080".to_string(),
    "20,http://proxy2:8080".to_string(),
])?;

let downloader = M3u8Downloader::new(
    "https://example.com/master.m3u8".to_string(),
    PathBuf::from("output.mp4"),
    PathBuf::from("temp"),
    15,
    true,
    Some(proxy_config),
    5,
    None,
    vec!["Authorization: Bearer token".to_string()],
    vec!["ad\\.".to_string()],
    true,
);

downloader.download().await?;
```

## 功能特性说明

### 嵌套播放列表支持
- **自动检测**: 自动识别主播放列表和媒体播放列表
- **质量选择**: 自动选择最佳质量的变体流
- **手动选择**: 支持手动指定变体流索引

### 下载优化
- **并发下载**: 多线程并发下载视频片段
- **断点续传**: 自动重试失败的片段
- **进度显示**: 实时显示下载进度和状态

### 网络功能
- **代理支持**: 多代理负载均衡
- **请求头定制**: 自定义 HTTP 请求头
- **广告过滤**: 基于正则表达式的广告片段过滤

### 合并选项
- **简单合并**: 快速合并，兼容大多数播放器
- **FFmpeg 合并**: 高质量合并，100% 播放器兼容

## 常见问题

### Q: 下载速度慢怎么办？
A: 可以增加 `--concurrent` 参数值，但不建议超过 20，以免被服务器限制。

### Q: 如何提高兼容性？
A: 使用 `--use-ffmpeg` 参数可以获得最佳的播放器兼容性。

### Q: 支持哪些视频格式？
A: 支持标准的 M3U8 格式，输出为 MP4 格式。

### Q: 如何处理认证保护的视频？
A: 使用 `--header` 参数添加认证信息。

### Q: 如何调试下载问题？
A: 使用 `--keep-temp` 参数保留临时文件进行分析。

## 注意事项

1. **网络环境**: 根据网络状况调整并发数量
2. **磁盘空间**: 确保有足够空间存储临时文件和最终视频
3. **权限**: 某些视频可能需要特殊认证
4. **格式支持**: 目前支持标准 M3U8 格式
5. **FFmpeg**: 使用 `--use-ffmpeg` 需要系统安装 FFmpeg

## 获取帮助

运行以下命令查看完整帮助信息：
```bash
rust-m3u8 --help
```

查看具体参数说明：
```bash
rust-m3u8 --help <参数名>