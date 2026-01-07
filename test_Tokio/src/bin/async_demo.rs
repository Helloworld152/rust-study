use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let start = std::time::Instant::now();

    // 1. 创建任务 1 (把它扔给 Tokio 调度器，不等待它完成)
    let task1 = tokio::spawn(async {
        fetch_url("https://www.baidu.com").await
    });

    // 2. 创建任务 2
    let task2 = tokio::spawn(async {
        fetch_url("https://www.google.com").await
    });

    // 此时两个任务都在后台跑了...

    // 3. 等待它们出结果 (Join)
    // 这里的 unwrap 是解开 JoinHandle 的 Result
    let res1 = task1.await.unwrap();
    let res2 = task2.await.unwrap();

    println!("总耗时: {:?}", start.elapsed()); // 结果大约是 1.00x 秒
}

async fn fetch_url(url: &str) -> String {
    sleep(Duration::from_secs(1)).await;
    println!("完成: {}", url);
    "Done".into()
}