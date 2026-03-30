use tokio::time::{sleep, Duration, Instant};

//sync fn不会立刻执行，调用它只是返回一个Future
//.await - 驱动上述 Future 执行，没有.await，Future则永远不会执行
async fn task(name: &str, secs: u64) -> String {
    println!("  ⏳ {name} 开始...");
    sleep(Duration::from_secs(secs)).await;
    println!("  ✅ {name} 完成！");
    format!("{name} 的结果")
}

pub async fn run() {
    // 顺序执行：2 + 3 = 5秒
    println!("【顺序执行】");
    let start = Instant::now();
    let a = task("泡茶", 2).await;//这里才真正执行，等待2秒后出结果
    let b = task("烤面包", 3).await;//这里才真正执行，等待3秒后出结果
    println!("  拿到: {a}, {b}");
    println!("  耗时: {:.1}秒\n", start.elapsed().as_secs_f64());

    // 并发执行：max(2, 3) = 3秒
    println!("【并发执行】");
    let start = Instant::now();
    //并发等待多个Future执行，泡茶和烤面包同时执行
    let (a, b) = tokio::join!(task("泡茶", 2), task("烤面包", 3));
    println!("  拿到: {a}, {b}");
    println!("  耗时: {:.1}秒\n", start.elapsed().as_secs_f64());
}
