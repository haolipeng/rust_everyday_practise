// 默认私有，外部无法访问
fn _secret() {
    println!("我是私有的");
}

// pub 公开，外部才能调用
pub fn hello(name: &str) {
    println!("Hello, {name}!");
}
