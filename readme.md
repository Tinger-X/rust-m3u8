# Rust M3U8 下载器

一个用 Rust 编写的高性能 M3U8 视频下载器，支持嵌套播放列表解析和多种高级功能。

## 🚀 功能特性

### 核心功能
- ✅ **标准 M3U8 播放列表解析** - 完整支持 M3U8 格式规范
- ✅ **嵌套播放列表支持** - 自动识别主播放列表和多个变体流
- ✅ **多线程并发下载** - 高性能并行下载视频片段
- ✅ **智能广告过滤** - 基于正则表达式的广告片段检测和过滤
- ✅ **代理负载均衡** - 多代理服务器权重轮询
- ✅ **自定义请求头** - 支持认证和自定义 HTTP 头
- ✅ **断点续传** - 自动重试失败的下载任务

### 高级特性
- 🎯 **智能质量选择** - 自动选择最佳质量的变体流
- 🔧 **多种合并模式** - 简单合并和 FFmpeg 高质量合并
- 📊 **实时进度显示** - 详细的下载进度和状态信息
- 🛡️ **错误恢复** - 强大的错误处理和重试机制
- 📁 **本地文件支持** - 支持本地 M3U8 文件处理

## 📦 安装

### 从源码安装
```bash
git clone https://github.com/Tinger-X/rust-m3u8.git
cd rust-m3u8
cargo install --path .
```

### 直接运行
```bash
cargo run -- [参数]
```

## 🎯 快速开始

### 基本下载
```bash
# 下载标准 M3U8 视频
rust-m3u8 https://example.com/playlist.m3u8

# 下载嵌套播放列表
rust-m3u8 https://example.com/master.m3u8 -o high_quality_video
```

### 高级下载
```bash
# 使用代理和广告过滤
rust-m3u8 https://example.com/playlist.m3u8 \
  -p "10,http://proxy1:8080" \
  -p "15,http://proxy2:8080" \
  -f "ad\\." \
  -f "tracking\\." \
  --use-ffmpeg
```

## 📚 使用示例

### 示例程序
项目提供了多个示例程序，展示不同功能的使用方式：

```bash
# 运行基础使用示例
cargo run --example basic_usage

# 运行高级功能示例
cargo run --example advanced_features

# 运行本地文件示例
cargo run --example local_file_usage

# 运行嵌套播放列表示例
cargo run --example nested_m3u8
```

### 程序库使用
```rust
use rust_m3u8::*;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    Ok(())
}
```

## 🏗️ 项目架构

### 模块结构
```
src/
├── lib.rs              # 库入口和公共接口
├── types.rs            # 核心数据类型定义
├── downloader.rs       # 下载器主逻辑实现
├── merger.rs           # 视频合并功能
├── proxy.rs           # 代理配置和管理
├── error.rs           # 错误类型定义
└── parser/            # 解析器模块
    ├── mod.rs         # 模块导出
    ├── content_parser.rs # 内容解析器
    ├── master_parser.rs # 主播放列表解析器
    ├── media_parser.rs  # 媒体播放列表解析器
    └── nested_parser.rs # 嵌套播放列表解析器
```

### 核心组件
1. **M3u8Downloader** - 主要的下载器接口
2. **NestedParser** - 嵌套播放列表解析器
3. **ProxyConfig** - 代理配置管理
4. **VideoMerger** - 视频合并器
5. **M3u8Playlist** - 播放列表数据结构

## 🧪 测试

项目包含完整的测试套件：

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_m3u8_parser
cargo test test_downloader
cargo test test_integration

# 运行测试并显示输出
cargo test -- --nocapture
```

### 测试覆盖
- ✅ 单元测试 - 各个模块的独立测试
- ✅ 集成测试 - 端到端的功能测试
- ✅ 解析器测试 - M3U8 格式解析测试
- ✅ 下载器测试 - 下载流程测试

## 🔧 开发指南

### 代码规范
- 遵循 Rust 官方编码规范
- 使用 clippy 进行代码检查
- 完整的文档注释

### 构建和运行
```bash
# 开发构建
cargo build

# 发布构建
cargo build --release

# 代码检查
cargo clippy

# 格式检查
cargo fmt --check
```

## 📖 文档

### 在线文档
```bash
# 生成本地文档
cargo doc --open
```

### 使用指南
详细的使用说明请参考 [examples/usage.md](examples/usage.md) 文件。

## 🤝 贡献指南

欢迎贡献代码！请遵循以下步骤：

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 打开 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🐛 问题反馈

如果您遇到任何问题，请通过以下方式反馈：

- [创建 Issue](https://github.com/Tinger-X/rust-m3u8/issues)
- 发送邮件到项目维护者

## 🙏 致谢

感谢所有为这个项目做出贡献的开发者！

---

**Rust M3U8 下载器** - 让 M3U8 视频下载变得简单高效！ 🎥
