# Rust：`Option::map` 与 `Option::and_then` 小演示

面向教程分享：用可运行的小例子说明两种组合子的区别，并贴近从 `serde_json::Value` 里取字段的写法（与 AgentSight `SslRunner` 解析 `pid` 的链式代码同思路）。

## 运行

```bash
cd /home/work/ebpf-tutorial/src/rust-option-map-and-then-demo
cargo run
```

## 核心区别（一句话）

| 方法 | 闭包返回 | 典型用途 |
|------|----------|----------|
| `map(f)` | **任意类型 `T`**，整体变成 `Option<T>` | 把「已有值」做变换（如 `u64` → `u32`） |
| `and_then(f)` | **`Option<U>`** | 下一步可能「没有值」，需要把多层可能失败的操作**拍平**成一层 `Option` |

## 常见坑

在 `map` 的闭包里若返回 `Option<_>`，会得到 `Option<Option<_>>`。需要继续「可能为空」的步骤时，应使用 `and_then`。

## 与 `SslRunner` 的对照

下面这段逻辑与本 demo 中 `demo_ssl_style_pid` / `demo_chain_like_ssl_runner` 一致：

```rust
json_value
    .get("pid")           // Option<&Value>
    .and_then(|v| v.as_u64())  // 只有 JSON 数字才是 Some(u64)
    .map(|p| p as u32)         // 有数字再转成 u32
```

## 许可证

示例代码可按你教程需要自由引用；若仓库有统一许可证，请与主仓库保持一致。
