pub fn run() {
    println!("===== while let：循环取值 =====");

    // pop() 返回 Option<T>
    // 有元素 → Some(值)
    // 空了   → None
    let mut stack = vec![1, 2, 3];

    // 只要 pop() 还能拿到 Some，就继续循环
    // 一旦返回 None，自动停下
    while let Some(top) = stack.pop() {
        println!("弹出: {}", top);
    }
    println!("栈空了！");

    println!("---");

    // 实际场景：逐行处理，遇到空行停止
    let lines = vec!["第一行", "第二行", "第三行", "", "这行不会被处理"];
    let mut iter = lines.iter();

    // next() 也返回 Option
    while let Some(&line) = iter.next() {
        if line.is_empty() {
            println!("遇到空行，停止");
            //break;
        }
        println!("处理: {}", line);
    }

    println!();
}