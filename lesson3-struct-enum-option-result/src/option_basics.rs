/// Option：值可能有，也可能没有
fn find_even(numbers: &[i32]) -> Option<i32> {
    for &n in numbers {
        if n % 2 == 0 {
            return Some(n);  // 找到了
        }
    }
    None  // 没找到
}

pub fn run() {
    println!("===== 场景3：Option：有或没有 =====");

    match find_even(&[1, 3, 4, 7]) {
        Some(n) => println!("找到偶数: {}", n),
        None    => println!("没有偶数"),
    }

    match find_even(&[1, 3, 5]) {
        Some(n) => println!("找到偶数: {}", n),
        None    => println!("没有偶数"),
    }

    println!();
}