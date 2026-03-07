/// 不可变借用：计算长度
fn calculate_length(s: &String) -> usize {
    s.len()
}

/// 可变借用：添加后缀
fn append_suffix(s: &mut String, suffix: &str) {
    s.push_str(suffix);
}

pub fn run() {
    println!("===== 场景4：不可变借用 vs 可变借用 =====");
    let mut greeting = String::from("Hello");

    // 不可变借用：只读
    let len = calculate_length(&greeting);
    println!("\"{}\" 的长度是 {}", greeting, len);

    // 可变借用：可写
    append_suffix(&mut greeting, ", Rustacean!");
    println!("追加后: {}", greeting);
    println!();
}
