[中文帮助文档](readme.md)

# rust-m3u8

Terminal M3U8 video downloader written in Rust

## Features

- **Cross-platform**: Supports Windows(only tested on Windows 11), macOS (untested), and Linux (only tested on Ubuntu 22.04)
- **Multithreading**: Supports multithreaded downloading with a default of 20 threads, configurable via a configuration file
- **Resume interrupted downloads**: Allows resuming downloads after interruption
- **Error retry**: Supports retrying failed downloads with a default of 3 retries, configurable via a configuration file
- **Ad filtering**: Supports URL-based ad filtering and resolution-based ad filtering (only keep the resolution that appears most frequently in the complete video, **to be implemented**)
- **Weighted proxy pool**: Supports configuring a weighted proxy pool and randomly selects a proxy based on weight
- **Local m3u8**: Supports pre-downloading the m3u8 file locally and reading the video segment list directly from the local file

## Parameter Description

```bash
M3U8 Video Downloader

Usage: rust-m3u8 [OPTIONS] <SRC>

Arguments:
  <SRC>  M3U8 source file address, supports http(s) and local files

Options:
  -d, --dest <DEST>          Output video filename (without extension, default current time)
  -c, --config <CONFIG>      Path to the configuration file, default is the default configuration
  -b, --base-url <BASE_URL>  Base URL for concatenating TS segment URIs
  -H, --header <HEADERS>     Add or override HTTP request headers, format: Key:Value
  -h, --help                 Print help
  -V, --version              Print version
```

Example: (Priority: `Command Line Arguments` > `Configuration File` > `Default Values`)
```bash
# Basic usage
rust-m3u8 https://example.com/video.m3u8

# Specify output filename
rust-m3u8 -d my_video https://example.com/video.m3u8

# Specify configuration file
rust-m3u8 -c my_config.toml https://example.com/video.m3u8

# Add custom HTTP request headers
rust-m3u8 -H "User-Agent: MyApp/1.0" -H "Accept-Language: en-US,en;q=0.9" https://example.com/video.m3u8
```

## Configuration File

The configuration file is in toml format, example: [config_demo.toml](config_demo.toml), with the following specific descriptions:

```toml
# Rust M3U8 Downloader Configuration File Example

# 1. General System Configuration
[system]
# 1.1 Concurrent Download Workers
workers = 24
# 1.2 Failed Retry Times (0 means infinite retries)
retry = 3
# 1.3 Proxy Pool Configuration
#    Format: `[address, weight]`
#    Downloaders will randomly select a proxy based on weight
#    Default: empty (use system network environment)
#    Example:
#    proxies = [
#        ["http://127.0.0.1:7890", 100],
#        ["http://127.0.0.1:7891", 50],
#    ]
proxies = []
# 1.4 Log Level, Options: trace, debug, info(default), warn, error
log_level = "trace"
# 1.5 Base URL for TS Segment URIs, default is empty
#    When TS segment URIs are not absolute URLs, this base URL is required
#    Example:
#    base_url = "https://example.com/"
# base_url = ""

# 2. HTTP Request Headers Configuration
[headers]
User-Agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
Accept = "*/*"
Accept-Encoding = "gzip, deflate, br"
Connection = "keep-alive"

# 3. Ad Filtering Configuration
[filters]
# 3.1 URL Matching Regular Expression List, default is empty
#    Example:
#    url_patterns = [
#        "ad\\.example\\.com",
#        ".*_ad\\.ts"
#    ]
url_patterns = []
# 3.2 Resolution Filtering
#    When set to true, only keep the video segments with the highest resolution frequency
#    Default: false
resolution = false
```

## TODO

+ decode `ts` Bytes into frame size, supoort resolution ads filter
+ support other output video format
+ optimize cross platform build

## Notes

Currently, it has only been compiled and tested on Windows 11 and Ubuntu 22.04. Other platforms are yet to be provided by developers who have the conditions. Thank you.

### Build Guidance

```bash
# pull project into the target machine, eg.:
git clone git@github.com:Tinger-X/rust-m3u8.git
# normal test
cargo run -- https://vip.ffzy-play7.com/20230102/10794_9794026c/2000k/hls/mixed.m3u8
# with config file（refer: `readme.md` and `config_demo.toml`）
cargo run -- https://vip.ffzy-play7.com/20230102/10794_9794026c/2000k/hls/mixed.m3u8 -c config_demo.toml
# run build
cargo build --release
# share your executable: you are welcome to submit issues or pull requests
```
