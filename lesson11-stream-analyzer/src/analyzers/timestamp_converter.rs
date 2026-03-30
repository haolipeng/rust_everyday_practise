use super::Analyzer;
use crate::event::EventStream;
use async_trait::async_trait;
use futures::stream::StreamExt;

/// 纳秒 -> 毫秒 转换器
/// 对应 AgentSight: timestamp_normalizer.rs
pub struct TimestampConverter;

impl TimestampConverter {
    pub fn new() -> Self { Self }
}

#[async_trait]
impl Analyzer for TimestampConverter {
    async fn process(&mut self, stream: EventStream) -> EventStream {
        // stream.map(): 来一个事件处理一个，不攒批
        let converted = stream.map(|mut e| {
            e.timestamp /= 1_000_000;
            e
        });
        Box::pin(converted) // 包装成 EventStream 返回
    }

    fn name(&self) -> &str { "TimestampConverter" }
}
