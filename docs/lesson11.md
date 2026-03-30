# Lesson 11: trait + async Stream 流式分析器管道

> 基于 `lesson11-stream-analyzer` 项目，理解 AgentSight 中 Analyzer trait 的设计模式。

## 核心概念

Analyzer 是 AgentSight 流式处理管道的核心抽象：**接收事件流，处理后输出新的事件流**。

```text
事件源 → [Analyzer A] → [Analyzer B] → [Analyzer C] → 输出
         时间戳转换      来源过滤        终端打印
```

## 项目结构

```
src/
├── main.rs                        <- 入口，组装管道
├── event.rs                       <- Event + EventStream 类型定义
└── analyzers/
    ├── mod.rs                     <- Analyzer trait 定义 + 子模块声明
    └── timestamp_converter.rs     <- 一个最简实现
```

对应 AgentSight 源码：

| Demo 文件 | AgentSight 对应文件 |
|---|---|
| `event.rs` | `collector/src/framework/core/events.rs` |
| `analyzers/mod.rs` | `collector/src/framework/analyzers/mod.rs` |
| `analyzers/timestamp_converter.rs` | `collector/src/framework/analyzers/timestamp_normalizer.rs` |

---

## 第一步：Event 和 EventStream

**文件：** `src/event.rs`

```rust
#[derive(Debug, Clone)]
pub struct Event {
    pub source: String,
    pub timestamp: u64,
    pub message: String,
}

pub type EventStream = Pin<Box<dyn futures::Stream<Item = Event> + Send>>;
```

`EventStream` 从内到外拆解：

```text
Pin < Box < dyn Stream<Item=Event> + Send > >
 │      │     │                       │
 │      │     │                       └── 可跨线程传递（配合 tokio）
 │      │     └── trait object：不关心具体类型，只要是 Stream
 │      └── 堆分配：dyn 大小编译期未知，用指针包一层
 └── 钉住内存：防止异步流内部自引用失效
```

### ��什么不用 `Vec<Event>`？

| | `Vec<Event>` | `EventStream` |
|---|---|---|
| 处理方式 | 批处理，全部加载到内存 | 流式，来一个处理一个 |
| 内存占用 | 与事件总量成正比 | 恒定，只有当前事件 |
| 适用场景 | 数据量小、一次性处理 | 持续监控、数据量无上限 |

AgentSight 做的是 SSL 实时监控，可能运行数小时，事件无限产生，所以必须用流。

---

## 第二步：Analyzer trait 定义

**文件：** `src/analyzers/mod.rs`

```rust
#[async_trait]
pub trait Analyzer: Send + Sync {
    async fn process(&mut self, stream: EventStream) -> EventStream;
    fn name(&self) -> &str;
}
```

| 语法 | 含义 |
|---|---|
| `trait` | 接口定义，所有 Analyzer 必须实现这两个方法 |
| `Send + Sync` | 要求线程安全（tokio 会跨线程调度任务） |
| `#[async_trait]` | Rust 原生 trait 不支持 async fn，这个宏补上 |
| `process()` | 核心：接��流 → 返回流（管道模式） |
| `name()` | 返回分析器名称 |

模块组织要点：

```rust
mod timestamp_converter;                  // 声明子模块 → 找 timestamp_converter.rs
pub use timestamp_converter::TimestampConverter;  // 重导出 → 外部直接用
```

---

## 第三步：实现一个 Analyzer

**文件：** `src/analyzers/timestamp_converter.rs`

```rust
pub struct TimestampConverter;

#[async_trait]
impl Analyzer for TimestampConverter {
    async fn process(&mut self, stream: EventStream) -> EventStream {
        let converted = stream.map(|mut e| {
            e.timestamp /= 1_000_000;   // 纳秒 → 毫秒
            e
        });
        Box::pin(converted)
    }

    fn name(&self) -> &str { "TimestampConverter" }
}
```

实现一个 Analyzer 的固定三步：

1. `stream.map(|event| { ... })` — 用闭包逐个处理事件
2. 在闭包里修改 event 然后返回
3. `Box::pin(...)` — 包装成 EventStream 返回

---

## 第四步：组装运行

**文件：** `src/main.rs`

```rust
// Vec → 异步 Stream
let stream: EventStream = Box::pin(stream::iter(events));

// 过管道
let stream = analyzer.process(stream).await;

// 消费流
let results: Vec<_> = stream.collect().await;
```

运行输出：

```
-> TimestampConverter
  [1000ms] [ssl] GET /api/users
  [2000ms] [process] python3 started
  [3000ms] [ssl] POST /api/chat
```

---

## 速查表

```text
Pin<Box<dyn Stream + Send>>     异步事件流的 trait object 类型
Box::pin(stream)                把具体流类型包装成 EventStream
stream::iter(vec)               Vec 转异步 Stream
stream.map(|e| { ... })         逐个变换事件（最常用）
stream.filter(|e| { ... })     逐个过滤事件
stream.collect().await          消费流，收集为 Vec
#[async_trait]                  让 trait 支持 async fn
Send + Sync                     线程安全约束，配合 tokio
mod xxx;                        声明子模块
pub use xxx::Struct;            重导出，简化外部引用路径
```

## 一句话总结

> Analyzer trait 的核心就是 `stream in → stream out` 的管道模式，用 `stream.map()` 做变换、`Box::pin()` 包装返回，多个 Analyzer 串起来就是完整的流式处理管道。
