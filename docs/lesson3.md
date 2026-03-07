# Rust 结构体、枚举、Option 与 Result

> 基于 `lesson3-struct-enum-option-result` 项目的 4 个场景，掌握 Rust 的自定义类型和错误处理基础。

## 核心概念

| 概念 | 用途 | 类比 |
|------|------|------|
| `struct` | 把多个相关字段组合成一个类型 | 表单 / 数据卡片 |
| `enum` | 定义一组互斥的可能值 | 单选题选项 |
| `Option<T>` | 值可能有，也可能没有 | 快递：签收 / 空包裹 |
| `Result<T, E>` | 操作可能成功，也可能失败 | 考试：通过 / 挂科（附原因） |

---

## 场景 1：结构体（struct）

**文件：** `src/struct_basics.rs`

### 定义与构造

```rust
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
            age,            // 字段名和变量名一样时可以简写
            active: true,   // 默认值
        }
    }
}
```

### 方法：`&self` vs `&mut self`

```rust
impl User {
    /// &self：只读，不修改
    fn introduce(&self) {
        println!("我叫{}，今年{}岁", self.name, self.age);
    }

    /// &mut self：可以修改字段
    fn set_age(&mut self, new_age: u32) {
        self.age = new_age;
    }
}

let mut user = User::new("小明", 20);
user.introduce();       // 我叫小明，今年20岁
user.set_age(21);
user.introduce();       // 我叫小明，今年21岁
```

**要点：**

| 方法签名 | 含义 | 要求 |
|----------|------|------|
| `&self` | 只读借用自身 | 实例可以是不可变的 |
| `&mut self` | 可变借用自身 | 实例必须声明为 `let mut` |
| `self` | 消费自身（取得所有权） | 调用后实例不可再用 |

---

## 场景 2：枚举（enum）

**文件：** `src/enum_basics.rs`

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn move_player(dir: &Direction) {
    match dir {
        Direction::Up    => println!("向上��"),
        Direction::Down  => println!("向下走"),
        Direction::Left  => println!("向左走"),
        Direction::Right => println!("向右走"),
    }
}

let dir = Direction::Up;
move_player(&dir);
```

**要点：** 枚举定义一组互斥的变体。配合 `match` 使用时，编译器强制要求**穷举所有变体**，漏掉任何一个都无法编译通过。这让代码不会遗漏分支。

---

## 场景 3：Option —— 值可能有也可能没有

**文件：** `src/option_basics.rs`

```rust
fn find_even(numbers: &[i32]) -> Option<i32> {
    for &n in numbers {
        if n % 2 == 0 {
            return Some(n);  // 找到了，包在 Some 里
        }
    }
    None  // 没找到
}

match find_even(&[1, 3, 4, 7]) {
    Some(n) => println!("找到偶数: {}", n),   // 找到偶数: 4
    None    => println!("没有偶数"),
}

match find_even(&[1, 3, 5]) {
    Some(n) => println!("找到偶数: {}", n),
    None    => println!("没有偶数"),            // 没有偶数
}
```

**要点：** `Option<T>` 是 Rust 替代 `null` 的方案：

```text
Option<T>
├── Some(T)   → 有值，值是 T
└── None      → 没有值
```

使用 `Option` 的好处：编译器**强制**你处理"没有值"的情况，不会出现空指针异常。

---

## 场景 4：Result —— 操作可能成功也可能失败

**文件：** `src/result_basics.rs`

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("不能除以零！"))  // 失败，附带错误信息
    } else {
        Ok(a / b)                          // 成功，附带结果
    }
}

match divide(10.0, 3.0) {
    Ok(val)  => println!("10 / 3 = {:.2}", val),  // 10 / 3 = 3.33
    Err(msg) => println!("错误: {}", msg),
}

match divide(10.0, 0.0) {
    Ok(val)  => println!("10 / 0 = {:.2}", val),
    Err(msg) => println!("错误: {}", msg),          // 错误: 不能除以零！
}
```

**要点：** `Result<T, E>` 把"可能失败"写在类型签名里：

```text
Result<T, E>
├── Ok(T)    → 成功，值是 T
└── Err(E)   → 失败，错误信息是 E
```

| 对比 | Option | Result |
|------|--------|--------|
| 语义 | 有或没有 | 成功或失败 |
| 有值 | `Some(T)` | `Ok(T)` |
| 无值/失败 | `None` | `Err(E)`（附带错误原因） |
| 典型场景 | 查找、可选配置 | I/O、解析、除法等可能出错的操作 |

---

## 速查表

```text
struct User { name: String }        自定义数据结构
impl User { fn new() -> User }      关联函数（构造器）
impl User { fn foo(&self) }         方法（只读）
impl User { fn bar(&mut self) }     方法（可修改）

enum Color { Red, Green, Blue }     枚举，穷举所有可能

Option<T>  =  Some(T) | None        有或没有
Result<T,E> = Ok(T)   | Err(E)      成功或失败
```

## 一句话总结

> `struct` 和 `enum` 让你定义自己的类型，`Option` 消灭空指针，`Result` 让错误处理显式化——Rust 把"可能出错"写在类型里，编译器帮你兜底。
