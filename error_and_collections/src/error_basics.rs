use std::num::ParseIntError;

fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>()
}

fn add_two(a: &str, b: &str) -> Result<i32, ParseIntError> {
    //尝试解析，? 是关键，用在函数体中自动return Err
    //? 成功时，自动从Ok(值) 中取出值，赋给x或者赋给y
    //? 失败时，自动把Err(错误)返回给调用者，函数提前结束
    let x = a.parse::<i32>()?;
    let y = b.parse::<i32>()?;
    Ok(x + y)
}

pub fn run() {
    println!("===== 错误处理 =====\n");

    println!("【match 处理】");
    //使用match分别处理两种情况，返回 Ok走OK分支，返回Err走Err分支
    match parse_number("42") {
        Ok(n) => println!("  成功: {}", n),
        Err(e) => println!("  失败: {}", e),
    }
    match parse_number("abc") {
        Ok(n) => println!("  成功: {}", n),
        Err(e) => println!("  失败: {}", e),
    }

    println!("\n【? 运算符】");
    //{:?} vs {} 的区别
    //add_two函数返回的是 Result<i32, ParseIntError>，不是简单的数字类型，所以这里不能使用{}
    println!("  10 + 20 = {:?}", add_two("10", "20"));
    println!("  10 + abc = {:?}", add_two("10", "abc"));

    println!();
}
