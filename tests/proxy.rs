use reqwest;
use std::fs::File;
use std::io::Write;
use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 设置代理
    let proxy = reqwest::Proxy::all("113.46.231.8:24975")?;

    // 创建客户端
    let client = reqwest::Client::builder()
        .proxy(proxy)
        .connect_timeout(Duration::from_secs(30)) // 使用系统根证书（与requests一致）
        .timeout(Duration::from_secs(30))
        .build()?;

    // 获取baidu文件内容
    println!("正在获取baidu");
    let resp = client.get("https://www.baidu.com/").send().await?;
    let text = resp.text().await?;
    println!("{}", text);
    return Ok(());

    // 获取m3u8文件内容
    println!("正在获取m3u8文件...");
    let resp = client
        .get("https://vip.ffzy-plays.com/20251004/46047_0b3f0a04/3000k/hls/mixed.m3u8")
        .send()
        .await?;
    let text = resp.text().await?;
    println!("{}", text);
    return Ok(());

    // 下载ts文件
    println!("正在下载ts文件...");
    let seg_resp = client
        .get("https://vip.ffzy-plays.com/20251004/46047_0b3f0a04/3000k/hls/dd3e82f2df2b90bd317204f5f872815a.ts")
        .send()
        .await?;

    let content = seg_resp.bytes().await?;

    // 写入文件
    let mut file = File::create("./test.ts")?;
    file.write_all(&content)?;

    println!("文件下载完成: ./test.ts");

    Ok(())
}
