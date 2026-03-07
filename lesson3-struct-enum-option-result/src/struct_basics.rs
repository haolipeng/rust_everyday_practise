/// 定义一个结构体，就像自定义一种数据类型
struct User {
    name: String,
    age: u32,
    active: bool,
}

impl User {
   /// 构造函数（Rust 惯例叫 new）
   fn new(name: &str, age: u32) -> User {
        User {
            name: String::from(name),
            age: age,           // 字段名和变量名一样时可以简写
            active: true,  // 默认值
        }
   }

   /// 方法：&self 表示借用自己，只读
   fn introduce(&self) {
    println!("我叫{}，今年{}岁", self.name, self.age);
   }

    /// 方法：&mut self 表示可变借用，可以修改
    fn set_age(&mut self, new_age: u32) {
        self.age = new_age;
    }
}

pub fn run() {
    println!("===== 场景1：结构体 =====");

    let mut user = User::new("小明", 20);
    user.introduce();

    user.set_age(21);
    user.introduce();

    println!("是否活跃: {}", user.active);
    println!();
}
