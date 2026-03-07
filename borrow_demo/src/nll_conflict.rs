/// 可变借用：追加内容
fn append_it(s: &mut String, text: &str) {
    s.push_str(text);
}

pub fn run() {
    println!("===== 场景6：NLL 冲突演示 =====");
    let mut s = String::from("hello");

    let r = &s;                         // 不可变借用

    // ❌ 取消下面两行注释，编译器会报错：
    // append_it(&mut s, ", world");    // 可变借用，与 r 冲突！
    // println!("r 还在用: {}", r);     // r 还活着，借用重叠！

    // ✅ 正确写法：先用完 r，再创建可变借用
    println!("先用完 r: {}", r);        // r 最后一次使用，NLL 在此结束
    append_it(&mut s, ", world");       // 现在可变借用安全了
    println!("修改后: {}", s);
    println!();
}
