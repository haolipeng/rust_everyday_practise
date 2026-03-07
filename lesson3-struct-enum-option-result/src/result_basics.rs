/// Rust 的 Result 把"可能失败"写在类型签名里,一眼就知道这个函数可能失败
/// Result：操作可能成功，也可能失败
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        //把错误原因包在 Err 里返回
        Err(String::from("不能除以零！"))
    } else {
        //把计算结果包在 Ok 里返回
        Ok(a / b)
    }
}

pub fn run() {
    println!("===== 场景4: Result 成功或失败 =====");

    //{:.2} 是格式化语法，表示保留两位小数。
    match divide(10.0, 3.0) {
        Ok(val)  => println!("10 / 3 = {:.2}", val),
        Err(msg) => println!("错误: {}", msg),
    }

    match divide(10.0, 0.0) {
        Ok(val)  => println!("10 / 0 = {:.2}", val),
        Err(msg) => println!("错误: {}", msg),
    }

    println!();
}
