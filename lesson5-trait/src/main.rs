mod traits;
mod models;
mod trait_bound;

//知识点学习
//trait        → 定义行为(接口)："你能做什么"
//impl for     → 实现行为："我能做这个"
//trait bound  → 约束类型："我只接受能做 XX 的"

/*
use crate::traits::{Greet, Age};
│   │      │       │
│   │      │       └── 从这个模块中导入哪些东西（可以多个）
│   │      └────────── 模块名（对应 src/traits.rs 文件）
│   └───────────────── 当前项目的根（src/main.rs 或 src/lib.rs）
└──────────────────── 关键字：把路径引入当前作用域
*/

fn main() {
    trait_bound::run();
}
