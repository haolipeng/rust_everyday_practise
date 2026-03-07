//match:多条件分支
//1.match 必须穷尽所有情况
//2.match 能匹配范围
//3.match 是表达式，有返回值
//match 的模式里写变量名 = 把被匹配的值取个别名
//    n if n < 0 => ...  ↑
//    "把 num 的值叫做 n，如果 n < 0 就执行后面的代码"
enum Coin {
    One,
    Five,
    Ten,
}

fn coin_value(coin: &Coin) -> u32 {
    // match 必须覆盖所有可能，少一个都编译不过
    match coin {
        Coin::One  => 1,
        Coin::Five => 5,
        Coin::Ten  => 10,
    }
}

pub fn run() {
    println!("===== match：全面匹配 =====");

    // 1. 匹配枚举
    let coins = [Coin::One, Coin::Five, Coin::Ten];
    for coin in &coins {
        println!("硬币面值: {}", coin_value(coin));
    }

    // 2. 匹配数字范围
    let score = 100;
    let grade = match score {
        //范式语法，用数学区间表示就是 [90, 100]，
        //在match中只能使用在 match 里只能用 ..=，不能用 ..
        90..=100 => "优秀",     // 90 到 100
        70..=89  => "良好",     // 70 到 89
        60..=69  => "及格",     // 60 到 69
        _        => "不及格",   // _ 是通配符，兜底
    };
    println!("{}分 → {}", score, grade);

    // 3. 匹配时加条件（match guard）
    let num = 4;
    match num {
        n if n < 0  => println!("{} 是负数", n),
        n if n == 0 => println!("是零"),
        n if n % 2 == 0 => println!("{} 是正偶数", n),
        n => println!("{} 是正奇数", n),
    }

    println!();
}
