/// 返回拥有的数据，不会有问题
fn create_greeting(name: &str) -> String {
    let greeting = format!("你好, {}!", name);
    greeting  // 把数据交出去，安全
}

/// ❌ 如果试图返回引用呢？取消注释试试：
// fn bad_greeting(name: &str) -> &str {
//     let greeting = format!("你好, {}!", name);
//     &greeting  // greeting 马上就没了，引用指向空气！
// }

pub fn run() {
    println!("===== 场景3：不能引用已经消失的数据 =====");

    let msg = create_greeting("小明");
    println!("{}", msg);

    let msg2 = create_greeting("Rust 学习者");
    println!("{}", msg2);

    // 为什么 bad_greeting 不行？
    // greeting 是函数内部创建的临时数据
    // 函数结束 → greeting变量就销毁了
    // 返回 &greeting → 指向一块已经被回收的内存
    //
    // Rust 编译器直接拦住你，不让编译通过

    println!();
}
