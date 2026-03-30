mod event;
mod analyzers;

use event::{Event, EventStream};
use analyzers::{Analyzer, TimestampConverter};
use futures::stream::{self, StreamExt};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let events = vec![
        Event { source: "ssl".into(),     timestamp: 1_000_000_000, message: "GET /api/users".into() },
        Event { source: "process".into(), timestamp: 2_000_000_000, message: "python3 started".into() },
        Event { source: "ssl".into(),     timestamp: 3_000_000_000, message: "POST /api/chat".into() },
    ];

    // Vec -> 异步 Stream
    let stream: EventStream = Box::pin(stream::iter(events));

    let mut analyzer = TimestampConverter::new();
    println!("-> {}", analyzer.name());
    let stream = analyzer.process(stream).await;

    // 消费流，逐个打印
    let results: Vec<_> = stream.collect().await;
    for e in &results {
        println!("  [{}ms] [{}] {}", e.timestamp, e.source, e.message);
    }
}
