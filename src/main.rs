mod utils;
mod core;
use clap::Parser;

use utils::logger::*;
use utils::args::Cli;
use utils::config::AppConfig;
use core::m3u8::M3U8;
use utils::common::Funcs;
use utils::errors::Result;

fn main() {
    let cli = Cli::parse();
    let mut config = AppConfig::parse(&cli.config);
    set_global_level(config.system.log_level);
    cli.update_config_headers(&mut config);
    
    // 执行主流程
    if let Err(e) = run(&cli, &config) {
        error_fmt!("执行失败: {:?}", e);
    }
}

fn run(cli: &Cli, config: &AppConfig) -> Result<()> {
    // 创建M3U8实例
    let mut m3u8 = M3U8::new(&config);
    
    // 解析M3U8文件
    info_fmt!("正在解析M3U8文件: {}", cli.src);
    m3u8.parse(&cli.src)?;
    info_fmt!(
        "解析完成，找到 {} 个片段，其中 {} 个广告，{} 个需要下载",
        m3u8.segments.len(),
        m3u8.ads,
        m3u8.need_downloads
    );
    
    // 下载片段
    info_fmt!("开始下载片段，使用 {} 线程", config.system.workers);
    m3u8.download();
    info_fmt!("下载完成，成功: {}, 失败: {}", m3u8.downloaded, m3u8.errors);
    m3u8.filter_ads_by_size();
    
    // 生成输出文件名
    let filename = match &cli.dest {
        Some(name) => name.to_string(),
        None => Funcs::generate_default_filename()
    };
    let filepath = Funcs::ensure_extension(&filename, &cli.ext);
    
    // 合并视频
    info_fmt!("正在合并视频到: {}", filepath);
    m3u8.merge_to_file(&filepath)?;
    m3u8.cleanup()?;
    
    info_fmt!("任务完成");
    Ok(())
}
