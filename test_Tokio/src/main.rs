use tokio::time::{sleep, Duration};


#[tokio::main]
async fn main() {
    println!("开始: {:?}", std::time::Instant::now());
    // 虽然用了 async，但 res1 等完了才等 res2，还是串行的
    let res1 = fetch_url("https://www.baidu.com").await;
    let res2 = fetch_url("https://www.google.com").await;
    println!("结束: {:?}", std::time::Instant::now());
}

async fn fetch_url(url: &str) -> String {
    println!("正在请求: {}", url);
    // 注意：这里不能用 std::thread::sleep，那是阻塞线程的！
    // 必须用 tokio::time::sleep，它是让出 CPU 权力的。
    sleep(Duration::from_secs(1)).await; 
    format!("Response from {}", url)
}