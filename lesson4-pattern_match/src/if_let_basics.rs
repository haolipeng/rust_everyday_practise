pub fn run() {
    println!("===== if let：只关心一种情况 =====");

    let some_value: Option<i32> = Some(42);

    // match 写法：两种都要处理
    match some_value {
        Some(n) => println!("[match]  有值: {}", n),
        None    => {},  // 不关心，但必须写 😐
    }

    // if let 写法：只关心 Some，简洁很多
    if let Some(n) = some_value {
        println!("[if let] 有值: {}", n);
    }

    // 加 else 处理另一种情况
    let no_value: Option<i32> = None;
    if let Some(n) = no_value {
        println!("[if let] 有值: {}", n);
    } else {
        println!("[if let] 没有值");
    }

    println!();
}
