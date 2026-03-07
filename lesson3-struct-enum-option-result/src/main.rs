mod struct_basics;
mod enum_basics;
mod option_basics;
mod result_basics;

fn main() {
    struct_basics::run();
    enum_basics::run();
    option_basics::run();
    result_basics::run();

    println!("===== 全部完成 ✅ =====");
}
