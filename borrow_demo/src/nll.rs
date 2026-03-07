/// 不可变借用：读取内容
fn read_it(s: &String) {
    println!("读取: {}", s);
}

/// 可变借用：追加内容
fn append_it(s: &mut String, text: &str) {
    s.push_str(text);
}

pub fn run() {
    println!("===== 场景3：NLL - 借用在最后一次使用后结束 =====");
    let mut s = String::from("hello");

    // 借用开始-先用不可变借用读取
    read_it(&s);
    // 借用结束-↑ 不可变借用在这里结束（NLL）

    // 再用可变借用修改，完全合法
    append_it(&mut s, ", world");
    println!("最终结果: {}", s);
    println!();
}
