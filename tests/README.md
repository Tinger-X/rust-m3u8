# 测试文件说明

本目录包含 rust-m3u8 库的测试文件，用于验证库的功能正确性。

## 测试文件列表

### 单元测试
- **test_format_duration.rs** - 格式化持续时间功能测试
- **test_m3u8_parser.rs** - M3U8 解析器功能测试
- **test_downloader.rs** - 下载器功能测试

### 集成测试
- **test_integration.rs** - 端到端集成测试

## 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试文件
cargo test --test test_downloader
cargo test --test test_m3u8_parser
cargo test --test test_integration

# 运行特定测试用例
cargo test test_downloader_creation
cargo test test_parser_integration
```

## 测试覆盖率

测试覆盖了以下核心功能：
- M3U8 文件解析和验证
- 嵌套播放列表处理
- 下载器配置和错误处理
- 代理配置和负载均衡
- 视频合并功能
- 错误处理和异常情况

## 测试数据

测试使用模拟数据和本地文件进行，避免依赖外部网络资源。集成测试可能需要有效的 M3U8 URL 才能完全运行。