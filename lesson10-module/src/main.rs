//mod作用: 声明模块（告诉编译器去哪找文件）
mod greet; //声明模块 -> 告诉编译器去找src/greet.rs文件
mod math; //声明模块 -> 告诉编译器去找src/math/mod.rs文件

//use作用：引入路径,简化调用
use math::basic; //把math::basic引入当前作用域，之后basic函数直接可用

fn main() {
    //方式一：完整路径
    greet::hello("Rust"); //用 模块名::函数名 调用
    //不使用use，则每次都要写math::basic::,有点繁琐
    println!("1 + 2 = {}", math::basic::add(1, 2));

    //方式二：use后的短路径
    println!("1 + 2 = {}", basic::add(1, 2));
    println!("square of 3 is {}", basic::square(3));
}
