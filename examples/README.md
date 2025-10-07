# 示例文件说明

本目录包含 rust-m3u8 库的使用示例，展示了不同场景下的使用方法。

## 示例文件列表

### 基础示例
- **basic_usage.rs** - 基础 M3U8 下载器使用示例
- **local_file_usage.rs** - 本地文件 M3U8 下载示例

### 高级功能示例
- **advanced_features.rs** - 高级功能使用示例（代理、广告过滤等）
- **nested_m3u8.rs** - 嵌套 M3U8 解析功能测试

### 文档
- **usage.md** - 详细使用指南和 API 文档

## 运行示例

```bash
# 运行基础示例
cargo run --example basic_usage

# 运行高级功能示例
cargo run --example advanced_features

# 运行本地文件示例
cargo run --example local_file_usage

# 运行嵌套 M3U8 示例
cargo run --example nested_m3u8
```

## 注意事项

1. 示例中的下载功能默认被注释，避免意外下载
2. 需要替换为实际的 M3U8 URL 才能进行真实下载测试
3. 确保有足够的磁盘空间用于临时文件和输出文件