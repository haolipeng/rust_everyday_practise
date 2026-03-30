use std::pin::Pin;

/// 事件：管道中流转的数据单元
#[derive(Debug, Clone)]
pub struct Event {
    pub source: String,
    pub timestamp: u64,
    pub message: String,
}

/// 异步事件流类型别名，拆解：
///   dyn Stream<Item=Event> -- trait object，不关心具体类型，只要是 Stream
///   Box<...>               -- 堆分配，因为 dyn 大小编译期未知
///   Pin<...>               -- 钉住内存位置，防止异步流内部自引用失效
///   + Send                 -- 可跨线程传递（配合 tokio）
pub type EventStream = Pin<Box<dyn futures::Stream<Item = Event> + Send>>;
