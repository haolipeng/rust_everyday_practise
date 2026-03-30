/// 1. 闭包基本语法
pub fn closure_syntax() {
    println!("=== 闭包基本语法 ===");

    // 完整写法
    let add = |a: i32, b: i32| -> i32 { a + b };
    // 简写（类型推断 + 省略大括号）
    let mul = |a, b| a * b;

    println!("add(2,3) = {}", add(2, 3));
    println!("mul(4,5) = {}", mul(4, 5));
}

/// 2. 闭包捕获环境变量（不可变借用最常用）
pub fn closure_capture() {
    println!("\n=== 闭包捕获环境 ===");

    // 不可变借用（默认，最常用）
    let name = String::from("Rust");
    let greet = || println!("Hello, {name}");
    greet();
    println!("name 还能用: {name}"); // ✅ 只是借用，原变量不受影响

    // 另外两种了解即可：
    // 可变借用：let mut inc = || count += 1;
    // move 转移所有权：let f = move || println!("{data:?}");
    //   → move 在多线程和 async 中常见
}

/// 3. 闭包作为参数
pub fn closure_as_param() {
    println!("\n=== 闭包作为参数 ===");

    fn apply(x: i32, f: impl Fn(i32) -> i32) -> i32 {
        f(x)
    }

    let square = |x| x * x;
    let result = apply(5, square);
    println!("平方: {result}");
}
