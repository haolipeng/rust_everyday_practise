pub fn run() {
    println!("===== 场景2：借用的两条规则 =====");

    let mut s = String::from("hello");

    // ✅ 规则1：多个人可以同时「看」
    let r1 = &s;
    let r2 = &s;
    println!("两个人同时看: {}, {}", r1, r2);

    // ✅ 规则2：只能一个人「改」
    let r3 = &mut s;
    r3.push_str(", world");
    println!("一个人改完了: {}", r3);

    println!();
}
