use std::collections::HashMap;

fn demo_vec() {
    println!("【Vec】");

    let mut nums = vec![10, 20, 30];
    println!("  初始: {:?}", nums);

    nums.push(40);
    println!("  增加后: {:?}", nums);
    nums.pop();
    nums[0] = 100;
    println!("  增删改后: {:?}", nums);

    println!("  get(1): {:?}", nums.get(1));
    println!("  get(2): {:?}", nums.get(2));
    println!("  get(99): {:?}", nums.get(99));
}

fn demo_hashmap() {
    println!("\n【HashMap】");

    let mut scores = HashMap::new();
    scores.insert("小明", 90);
    scores.insert("小红", 85);
    println!("  初始: {:?}", scores);

    println!("  小明: {:?}", scores.get("小明"));
    println!("  小芳: {:?}", scores.get("小芳"));//小芳此时不在hashmap中

    //如果key不存在就插入，存在就什么都不做
    scores.entry("小芳").or_insert(88);
    scores.entry("小明").or_insert(0);
    println!("  entry 后: {:?}", scores);

    println!("  遍历:");
    for (name, score) in &scores {
        println!("    {}: {}", name, score);
    }
}

pub fn run() {
    println!("===== 集合 =====\n");
    demo_vec();
    demo_hashmap();
    println!();
}
