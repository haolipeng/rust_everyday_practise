mod match_basics;
mod if_let_basics;
mod while_let_basics;

fn main() {
    match_basics::run();
    if_let_basics::run();
    while_let_basics::run();

    println!("===== 全部完成 ✅ =====");
}
