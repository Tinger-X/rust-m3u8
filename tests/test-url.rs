use reqwest::Url;

pub fn url_parse(url: &str) {
    let res = Url::parse(url);
    assert!(res.is_ok(), "URL 解析失败: {:?}, 原始URL: {}", res.unwrap_err(), url);
    println!("URL 解析结果: {:?}", res.unwrap());
}

#[test]
fn test_url_01() {
    url_parse(
        "https://console.lookbook.top/api/yz_siyou/17597213100e42c1bbe258964648f0b2c46adad831.m3u8?OTgzNGJ1U2J
        qb0hFb1hLUlQ3UjZtVE1qSnFNQ3VwM2ljTDNSeXNtcytnMFd2bmx3WUtpR2ZxL0VEZ2NoYjhTQ2RKUkVQNzB5eTVVRUlMYVhNb05ycjZSN1
        Z6dTNhaHA1QkJUSVgxNnVRTGJ1dUlULzdSM0NBdzZ4blB5dUdldWRpdlFuTmU3SDZESm5ZUHJvdlZvWUZaQlowS040QTJKb25qNEtHbXNLb
        1JQZE9UUThCeEVoWVY5QW1mUDBqQXVIc1UwckRLMVRpWStXNVNUK0Q1bTNoMmx2d3dOTW4wakV0a2tDUkRhekRlKzFXcmJmd0JQRkxaNWZM
        aHV5K05MYWZn@gekai@29751d6df7479320338867acea73107b@gekai@NmNjOVEwMmMvSitDeTZ2V2FCNzFhbVZmdmdzZ0llSFE4MTFFT
        GFQUm1JTGVsM1N0MUxhdg==@gekai@NTVjOWRUNXVuRWIwTGN0YUVrK1JiUUVaWVYvSlA3c3cyMTFCUWJRRHNiNA==@gekai@YmViN0hHdH
        kvYUtPSnpFblRwTU85Wnh1dzhra041a2I4Z1NyRjFPSDN5Mk9YbUYwZGdVTFczNUlkWDQ1bmdtS1Yycy9BNUNsNEZIQ3A0QzkrUQ==@geka
        i@"
    );
}

#[test]
fn test_url_02() {
    url_parse(
        "https://shandong-01.ckhao.top/videos/202504/06/67f2959428f73df3ba7da77b/97a639/index0.ts?sign=17597213
        12-JNPLgK0PCX8WX8cmZAqi4q2YB-a2e2d002a5158f239ab62ab826cfa151"
    );
}

#[test]
fn test_url_03() {
    url_parse("//www.baidu.com");
}

#[test]
fn test_url_04() {
    url_parse("https://www.baidu.com");
}

#[test]
fn test_url_05() {
    url_parse("127.0.0.1:9090");
}
