mod reference_basics;
mod borrow_rules;
mod no_dangling;

fn main() {
    reference_basics::run();
    borrow_rules::run();
    no_dangling::run();

    println!("===== 全部完成 ✅ =====");
}
