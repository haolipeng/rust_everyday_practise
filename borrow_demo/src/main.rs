mod immutable_borrow;
mod mutable_borrow;
mod nll;
mod function_borrow;
mod nll_conflict;

fn main() {
    immutable_borrow::run();
    mutable_borrow::run();
    nll::run();
    function_borrow::run();
    nll_conflict::run();

    println!("===== 所有场景运行完毕 ✅ =====");
}
