/// 不可变借用：只读，不获取所有权
fn print_info(s: &String) {
    println!("内容: {}", s);
    println!("长度: {}", s.len());
}

pub fn run() {
    println!("===== 场景1：不可变借用 =====");
    let s = String::from("hello, rust");

    print_info(&s);               // 借出去看看
    println!("我还能用: {}", s);   // s 依然有效
    println!();
}
