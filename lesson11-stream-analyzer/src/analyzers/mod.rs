use crate::event::EventStream;
use async_trait::async_trait;

/// Analyzer trait —— 所有分析器的统一接口
/// 对应 AgentSight: collector/src/framework/analyzers/mod.rs
///
/// 与之前 Vec<Event> 版本的区别：
///   Vec<Event>   -- 批处理，必须全部加载到内存
///   EventStream  -- 流式处理，来一个处理一个，内存恒定
#[async_trait]
pub trait Analyzer: Send + Sync {
    async fn process(&mut self, stream: EventStream) -> EventStream;
    fn name(&self) -> &str;
}

mod timestamp_converter;
pub use timestamp_converter::TimestampConverter;
