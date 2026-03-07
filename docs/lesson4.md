# Rust 模式匹配：match、if let、while let

> 基于 `lesson4-pattern_match` 项目的 3 个场景，掌握 Rust 最强大的控制流工具——模式匹配。

## 核心概念

模式匹配是 Rust 的核心特性之一，用于根据数据的"形状"来执行不同的逻辑。

| 语法 | 适用场景 | 特点 |
|------|---------|------|
| `match` | 需要处理所有可能的情况 | 必须穷举，最全面 |
| `if let` | 只关心其中一种情况 | 简洁，可选 `else` |
| `while let` | 循环处理直到模式不匹配 | 配合迭代器和 `Option` |

---

## 场景 1：match —— 全面匹配

**文件：** `src/match_basics.rs`

### 匹配枚举

```rust
enum Coin {
    One,
    Five,
    Ten,
}

fn coin_value(coin: &Coin) -> u32 {
    match coin {
        Coin::One  => 1,
        Coin::Five => 5,
        Coin::Ten  => 10,
    }
}
```

**要点：** `match` 必须覆盖所有变体，少一个都编译不过。这保证不会遗漏任何分支。

### 匹配数字范围

```rust
let score = 85;
let grade = match score {
    90..=100 => "优秀",     // 90 到 100（闭区间）
    70..=89  => "良好",
    60..=69  => "及格",
    _        => "不及格",   // _ 是通配符，兜底
};
println!("{}分 → {}", score, grade);  // 85分 → 良好
```

**要点：**

| 语法 | 含义 |
|------|------|
| `90..=100` | 闭区间范围匹配，等价于数学中的 [90, 100] |
| `_` | 通配符，匹配其他所有情况（兜底） |

注意：`match` 中只能用 `..=`（包含两端），不能用 `..`（不包含右端）。

### match guard（匹配守卫）

```rust
let num = 4;
match num {
    n if n < 0      => println!("{} 是负数", n),
    n if n == 0     => println!("是零"),
    n if n % 2 == 0 => println!("{} 是正偶数", n),   // 命中: 4 是正偶数
    n               => println!("{} 是正奇数", n),
}
```

**要点：** `n if 条件` 的写法叫**匹配守卫**：
- `n` 绑定被匹配的值（给它一个名字）
- `if 条件` 是额外的判断，只有条件为 `true` 才匹配该分支

### match 的三个关键特性

1. **必须穷举** —— 漏掉任何情况都编译不过
2. **能匹配范围** —— `90..=100` 这样的区间
3. **是表达式** —— 有返回值，可以赋给变量（如上面的 `grade`）

---

## 场景 2：if let —— 只关心一种情况

**文件：** `src/if_let_basics.rs`

### match vs if let 对比

```rust
let some_value: Option<i32> = Some(42);

// match 写法：两种都要处理
match some_value {
    Some(n) => println!("[match]  有值: {}", n),
    None    => {},  // 不关心，但必须写
}

// if let 写法：只关心 Some，简洁很多
if let Some(n) = some_value {
    println!("[if let] 有值: {}", n);
}
```

### 带 else 的 if let

```rust
let no_value: Option<i32> = None;

if let Some(n) = no_value {
    println!("有值: {}", n);
} else {
    println!("没有值");       // 命中这里
}
```

**要点：** `if let` 是 `match` 的语法糖，适用于只关心一种模式的场景：

| 场景 | 推荐用法 |
|------|---------|
| 需要处理所有分支 | `match` |
| 只关心一种情况 | `if let` |
| 关心一种情况 + 其他 | `if let ... else` |

---

## 场景 3：while let —— 循环取值

**文件：** `src/while_let_basics.rs`

### 配合 Vec::pop() 使用

```rust
let mut stack = vec![1, 2, 3];

// pop() 返回 Option<T>：有元素 → Some(值)，空了 → None
while let Some(top) = stack.pop() {
    println!("弹出: {}", top);   // 依次输出: 3, 2, 1
}
println!("栈空了！");
```

### 配合迭代器使用

```rust
let lines = vec!["第一行", "第二行", "第三行", "", "这行不会被处理"];
let mut iter = lines.iter();

// next() 返回 Option：有下一个 → Some(值)，没有了 → None
while let Some(&line) = iter.next() {
    if line.is_empty() {
        println!("遇到空行，停止");
    }
    println!("处理: {}", line);
}
```

**要点：** `while let` 的工作流程：

```text
循环开始
  ↓
stack.pop() 返回什么？
  ├── Some(top) → 执行循环体，继续下一轮
  └── None      → 退出循环
```

适用于不断从集合中取值、直到取完为止的场景。

---

## 三种模式匹配对比

```text
match value {              ← 穷举所有情况，最严格
    Pattern1 => ...,
    Pattern2 => ...,
    _        => ...,       ← 必须兜底
}

if let Pattern = value {   ← 只匹配一种，其余忽略
    ...
}

while let Pattern = expr { ← 循环匹配，直到不满足
    ...
}
```

## 速查表

```text
match x { ... }             穷举匹配，必须覆盖所有情况
90..=100                     闭区间范围模式
_                            通配符，匹配任意值
n if n > 0                   匹配守卫，绑定值 + 额外条件
match 是表达式               可以 let result = match x { ... };

if let Some(v) = opt { }    只关心 Some 的简写
if let Err(e) = res { }     只关心 Err 的简写

while let Some(v) = iter.next() { }   循环取值直到 None
while let Some(v) = stack.pop() { }   逐个弹出直到空
```

## 一句话总结

> `match` 穷举一切、`if let` 只取所需、`while let` 循环解包——三种模式匹配覆盖了从全面检查到简洁取值的所有场景。
