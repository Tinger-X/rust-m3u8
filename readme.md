[English Readme](readme_en.md)

# rust-m3u8

终端M3U8视频下载器，使用rust编写

## 特性

- **跨平台**：支持Windows(仅测试windows 11)、macOS(未测试)、Linux(仅测试ubuntu 22.04)
- **多线程**：支持多线程下载，默认线程数为10，通过配置文件指定
- **断点续传**：支持断点续传，下载中断后可以继续下载
- **错误重试**：支持下载失败重试，默认重试3次，通过配置文件指定
- **广告过滤**：支持URL过滤广告、分辨率过滤广告（仅保留完整视频中出现频次最高的分辨率，**待实现**）
- **权重代理池**：支持配置权重代理池，根据权重随机选择代理
- **本地m3u8**：支持预先下载m3u8文件到本地，直接从本地读取视频片段列表

## 参数说明

```bash
M3U8视频下载器

Usage: rust-m3u8 [OPTIONS] <SRC>

Arguments:
  <SRC>  M3U8源文件地址，支持http(s)以及本地文件

Options:
  -d, --dest <DEST>          下载完成后输出的视频文件名（不含扩展，默认当前时间）
  -c, --config <CONFIG>      配置文件路径，不指定则使用默认配置
  -b, --base-url <BASE_URL>  基础url，用于拼接ts片段uri
  -H, --header <HEADERS>     添加或覆盖HTTP请求头信息，格式: Key:Value
  -h, --help                 Print help
  -V, --version              Print version
```

示例：（优先级：命令行参数 > 配置文件 > 程序默认值）
```bash
# 基本用法
rust-m3u8 https://example.com/video.m3u8

# 指定输出文件名
rust-m3u8 -d my_video https://example.com/video.m3u8

# 指定配置文件
rust-m3u8 -c my_config.toml https://example.com/video.m3u8

# 添加自定义HTTP请求头
rust-m3u8 -H "User-Agent: MyApp/1.0" -H "Accept-Language: en-US,en;q=0.9" https://example.com/video.m3u8
```

## 配置文件

配置文件采用toml格式，示例：[config_demo.toml](config_demo.toml)，具体说明如下：

```toml
# Rust M3U8 下载器配置文件示例

# 1. 通用系统配置
[system]
# 1.1 并发下载数量，程序默认10
workers = 10
# 1.2 失败重试次数（0表示无限重试，程序默认3）
retry = 3
# 1.3 代理池，配置方式为：`[地址, 权重]` 下载时按照权重分配，程序默认不开启（使用系统网络环境），示例：
#    proxies = [
#        ["http://127.0.0.1:7890", 100],
#        ["http://127.0.0.1:7891", 50],
#    ]
proxies = []
# 1.4 日志等级：trace, debug, info(默认), warn, error
log_level = "trace"
# 1.5 ts地址的基础url，当ts连接不是绝对url时需要启用，程序默认不指定
# base_url = ""

# 2. HTTP请求头配置，程序默认值如下
[headers]
User-Agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
Accept = "*/*"
Accept-Encoding = "gzip, deflate, br"
Connection = "keep-alive"

# 3. 广告过滤配置
[filters]
# 3.1 URL匹配正则表达式列表，示例（默认为空）:
#    url_patterns = [
#        "ad\\.example\\.com",
#        ".*_ad\\.ts"
#    ]
url_patterns = []
# 3.2 分辨率匹配，取true时仅保留分辨率出现频次最高的视频片段，移除其它分辨率片段，默认不开启
resolution = false
```