# Rust 借用机制教程

> 基于 `borrow_demo` 项目的 5 个场景，快速掌握 Rust 的所有权与借用规则。

## 核心规则

Rust 在编译期通过 **借用检查器 (Borrow Checker)** 保证内存安全，规则只有三条：

1. 同一时刻，要么有 **多个不可变引用** `&T`，要么有 **一个可变引用** `&mut T`，二者不能共存
2. 引用必须始终有效（不能悬垂）
3. 借用期间，所有者不能转移或销毁数据

---

## 场景 1：不可变借用

**文件：** `src/immutable_borrow.rs`

```rust
fn print_info(s: &String) {       // 参数类型是 &String，只读借用
    println!("内容: {}", s);
    println!("长度: {}", s.len());
}

let s = String::from("hello, rust");
print_info(&s);                   // 通过 &s 借出，不转移所有权
println!("我还能用: {}", s);       // s 依然有效
```

**要点：** `&s` 是不可变引用，函数只能读不能改。调用结束后 `s` 仍然可用，所有权没有发生转移。

---

## 场景 2：可变借用

**文件：** `src/mutable_borrow.rs`

```rust
fn add_world(s: &mut String) {    // &mut String: 可变借用
    s.push_str(", world!");
}

let mut s = String::from("hello"); // 变量本身必须是 mut
add_world(&mut s);                 // 传入 &mut s
println!("修改后: {}", s);          // 输出: hello, world!
```

**要点：** 想通过引用修改数据，需要满足两个条件：

| 条件 | 说明 |
|------|------|
| `let mut s` | 变量声明为可变 |
| `&mut s` | 传递可变引用 |

缺少任何一个，编译器都会报错。

---

## 场景 3：NLL（非词法生命周期）

**文件：** `src/nll.rs`

```rust
let mut s = String::from("hello");

read_it(&s);                      // 不可变借用在这一行结束（NLL）
// --- 不可变借用的生命周期到此为止 ---

append_it(&mut s, ", world");     // 可变借用，合法！
println!("最终结果: {}", s);
```

**要点：** Rust 2021 起，借用的生命周期在 **最后一次使用处** 结束，而非作用域末尾。这就是 NLL (Non-Lexical Lifetimes)。上面的代码中，不可变借用在 `read_it` 调用后就结束了，因此后续的可变借用不会冲突。

---

## 场景 4：函数中不可变与可变借用对比

**文件：** `src/function_borrow.rs`

```rust
fn calculate_length(s: &String) -> usize {  // 不可变借用: 只读
    s.len()
}

fn append_suffix(s: &mut String, suffix: &str) {  // 可变借用: 可写
    s.push_str(suffix);
}

let mut greeting = String::from("Hello");
let len = calculate_length(&greeting);        // 只读
append_suffix(&mut greeting, ", Rustacean!"); // 可写
```

**要点：** 根据函数需要选择引用类型 —— 只需要读取用 `&T`，需要修改用 `&mut T`。参数 `suffix: &str` 是字符串切片的不可变引用，这是 Rust 中传递字符串的惯用方式。

---

## 场景 5：NLL 冲突演示

**文件：** `src/nll_conflict.rs`

### 错误写法

```rust
let mut s = String::from("hello");
let r = &s;                           // 不可变借用开始

append_it(&mut s, ", world");         // 错误! 可变借用与 r 冲突
println!("r 还在用: {}", r);           // r 还活着 -> 编译失败
```

编译器报错 `E0502`: 不可变借用和可变借用的生命周期重叠。

### 正确写法

```rust
let mut s = String::from("hello");
let r = &s;

println!("先用完 r: {}", r);           // r 最后一次使用，NLL 在此结束
append_it(&mut s, ", world");         // 现在可变借用安全了
println!("修改后: {}", s);
```

**要点：** 只要确保不可变引用在可变借用之前 **用完**，就不会冲突。

---

## 速查表

```text
let s = String::from("hi");

&s          不可变借用    可以有多个    只能读
&mut s      可变借用      只能有一个    可以改    变量必须声明为 mut
s (move)    所有权转移    原变量失效    完全拥有
```

## 一句话总结

> 借用 = 临时借出数据的访问权，编译器确保"读写互斥、写写互斥"，从而在零运行时开销下保证内存安全。
