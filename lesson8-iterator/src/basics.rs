/// 1. 基本迭代 —— iter() 借用，原数据不受影响
pub fn iter_basic() {
    println!("=== 基本迭代 ===");

    // vec! 是宏（带 ! 标志），可接受任意数量参数，编译时展开
    let nums = vec![1, 2, 3, 4, 5];

    for n in nums.iter() {
        print!("{n} ");
    }
    println!();
    println!("nums 还能用: {nums:?}"); // ✅ iter() 只是借用
}

/// 2. map —— 逐个元素转换
///
/// nums.iter()         → 创建迭代器，每个元素类型是 &i32（借用）
///     .map(|&x| ...)  → |&x| 是模式匹配解引用，把 &i32 解开为 i32，再执行 x * 2
///     .collect()      → 消费迭代器，收集为集合（惰性迭代在此刻才真正执行）
///
/// Vec<i32> 类型标注告诉 collect() 要收集成什么类型
pub fn iter_map() {
    println!("\n=== map 转换 ===");

    let nums = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = nums.iter().map(|&x| x * 2).collect();

    println!("原始: {nums:?}");
    println!("翻倍: {doubled:?}");
}

/// 3. filter —— 按条件过滤
///
/// nums.iter()             → 迭代器产出 &i32
///     .filter(|&&x| ...)  → filter 再借用一层，闭包收到 &&i32，所以用 |&&x| 解两层
///     .collect()          → filter 只筛选不转换，保留的还是 &i32，所以结果是 Vec<&i32>
pub fn iter_filter() {
    println!("\n=== filter 过滤 ===");

    let nums = vec![1, 2, 3, 4, 5];
    let evens: Vec<&i32> = nums.iter().filter(|&&x| x % 2 == 0).collect();

    println!("原始: {nums:?}");
    println!("偶数: {evens:?}");
}
