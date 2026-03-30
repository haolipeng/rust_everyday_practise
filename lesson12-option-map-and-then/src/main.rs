//! # `Option::map` 与 `Option::and_then` 演示
//!
//! 对应常见写法：从 `serde_json::Value` 里按路径取字段，链式处理，最后 `unwrap_or` / `expect`。
//! 与 AgentSight `SslRunner` 里解析 `pid` 的思路一致（见文末「实战对照」）。

use serde_json::{json, Value};

fn main() {
    println!("=== 1. map：把 Some 里的值变换一下，仍是 Option ===\n");
    demo_map();

    println!("\n=== 2. and_then：下一步仍可能是“没有值”（返回 Option）===\n");
    demo_and_then();

    println!("\n=== 3. map 里如果返回 Option？会嵌套成 Option<Option<...>>（常见坑）===\n");
    demo_map_nested_option_pitfall();

    println!("\n=== 4. 组合：先 and_then 再 map（类似 ssl 里取 pid）===\n");
    demo_chain_like_ssl_runner();

    println!("\n=== 5. 实战对照：SslRunner 里 pid 解析链 ===\n");
    demo_ssl_style_pid();
}

fn demo_map() {
    let x: Option<i32> = Some(10);
    let y = x.map(|n| n * 2); // Some(20)
    println!("Some(10).map(|n| n * 2) => {:?}", y);

    let none: Option<i32> = None;
    let z = none.map(|n| n * 2); // None，闭包不会执行
    println!("None.map(|n| n * 2)    => {:?}", z);
}

fn demo_and_then() {
    // 模拟「先解析字符串为数字，失败则没有值」
    fn parse_positive(s: &str) -> Option<u32> {
        s.parse().ok().filter(|&n: &u32| n > 0)
    }

    let a = Some("42");
    let step = a.and_then(|s| parse_positive(s));
    println!(r#"Some("42").and_then(parse_positive) => {:?}"#, step);

    let b = Some("0");
    println!(
        r#"Some("0").and_then(parse_positive) => {:?}  // 过滤后变 None"#,
        b.and_then(|s| parse_positive(s))
    );

    let c: Option<&str> = None;
    println!(
        "None.and_then(parse_positive)       => {:?}",
        c.and_then(|s| parse_positive(s))
    );
}

fn demo_map_nested_option_pitfall() {
    fn maybe_len(s: &str) -> Option<usize> {
        if s.is_empty() {
            None
        } else {
            Some(s.len())
        }
    }

    let x = Some("hello");
    // 错误示范：map 里返回 Option，得到 Option<Option<usize>>
    let nested: Option<Option<usize>> = x.map(|s| maybe_len(s));
    println!("Some(\"hello\").map(maybe_len) => {:?}  (嵌套 Option)", nested);

    // 正确：用 and_then 扁平化
    let flat: Option<usize> = x.and_then(|s| maybe_len(s));
    println!("Some(\"hello\").and_then(maybe_len) => {:?}", flat);
}

fn demo_chain_like_ssl_runner() {
    let v = json!({ "pid": 188705, "comm": "claude" });

    // 模仿：get -> as_u64 -> 转成 u32
    let pid: Option<u32> = v
        .get("pid")
        .and_then(|x| x.as_u64())
        .map(|p| p as u32);

    println!("json = {}", v);
    println!("chain result: {:?}", pid);
}

fn demo_ssl_style_pid() {
    // 与 ssl.rs 中逻辑等价（这里用 expect 代替 panic 信息，便于阅读）
    let ok: Value = json!({ "pid": 188705, "comm": "claude", "timestamp_ns": 1234567890u64 });

    let pid = ok
        .get("pid")
        .and_then(|v| v.as_u64())
        .map(|p| p as u32)
        .expect("Missing pid field in ssl event");

    println!("ok event -> pid = {}", pid);

    let missing_pid: Value = json!({ "comm": "claude" });

    let result = missing_pid
        .get("pid")
        .and_then(|v| v.as_u64())
        .map(|p| p as u32);

    println!("missing pid -> Option = {:?} (若为 None 则原代码会 panic)", result);
}
