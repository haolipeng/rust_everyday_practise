pub fn run() {
    println!("===== 场景1：引用是什么 =====");

    let s1 = String::from("hello");
    let r1 = &s1;  // r1 借用 s1，不拥有数据

    println!("s1: {}", s1);   // ✅ s1 还是主人
    println!("r1: {}", r1);   // ✅ r1 只是看看

    // 类比：s1 是书的主人，r1 只是借来翻翻
    // r1 离开时，书不会消失，因为书不是它的
    println!();
}
