use crate::traits::{Greet, Age};
use crate::models::{Chinese, American};

//传入的函数参数person,必须实现了 Greet 这个 trait
fn greet(person: &impl Greet) {
    println!("  {}", person.hello());
}

//传入的函数参数person,必须实现了 Greet 这个 trait + Age 这个 trait,是同时满足的关系，不是单一一个
fn greet_and_age(person: &(impl Greet + Age)) {
    println!("  {}, 年龄: {}", person.hello(), person.age());
}

pub fn run() {
    println!("===== trait bound =====\n");

    let c = Chinese { name: "小明".into() };
    let a = American { name: "Tom".into() };

    println!("【单个约束】");
    greet(&c);
    greet(&a);

    println!("\n【多个约束: Greet + Age】");
    greet_and_age(&c);

    println!();
}
