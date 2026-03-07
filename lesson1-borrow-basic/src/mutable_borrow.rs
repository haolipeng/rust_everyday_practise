/// 可变借用：可以修改原数据
fn add_world(s: &mut String) {
    s.push_str(", world!");
}

pub fn run() {
    println!("===== 场景2：可变借用 =====");
    //这里不用mut修饰s行不行？
    let mut s = String::from("hello");

    println!("修改前: {}", s);
    add_world(&mut s);
    println!("修改后: {}", s);
    println!();
}
