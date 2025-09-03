use tokio::sync::broadcast::{self, channel};

/**
 * 使用 Tokio 的广播通道，允许多个消费者订阅同一事件流。
 * 每个消费者都可以独立地接收事件，而不会影响其他消费者。
 * 当生产者发送事件时，所有订阅者都会收到该事件。
 */
#[derive(Debug, Clone)]
pub struct SseSender {
    pub tx: broadcast::Sender<String>,
}

impl SseSender {
    /// 创建一个新的 SseSender 实例
    ///
    /// 该方法会初始化一个容量为 100 的广播通道，用于发送服务器发送事件（SSE）
    /// 广播通道允许多个接收者订阅同一个消息流，每个接收者都会收到相同的消息
    ///
    /// # 返回值
    /// 返回一个新的 SseSender 实例
    pub fn new() -> Self {
        let (tx, _) = channel(100);
        Self { tx }
    }

    /// 创建一个新的广播通道接收者
    ///
    /// 该方法允许创建一个新的订阅者，用于接收通过 SseSender 发送的消息
    /// 每个接收者都会独立地收到所有发送的消息，不会影响其他接收者
    ///
    /// # 返回值
    /// 返回一个广播通道的接收者，可以用于接收消息
    pub fn create_channel(&self) -> broadcast::Receiver<String> {
        self.tx.subscribe()
    }

    /// 发送消息到所有订阅的客户端
    ///
    /// 该方法将消息发送到所有订阅的客户端，如果没有活动的接收者，则不会发送消息
    ///
    /// # 参数
    /// * `msg` - 要发送的消息
    ///
    /// # 返回值
    /// 返回发送结果，成功返回 Ok(())，失败返回 Err(错误信息)
    pub fn send(&self, msg: String) -> Result<(), String> {
        match self.tx.send(msg) {
            Ok(_) => Ok(()),
            Err(e) => {
                // 当没有活动的接收者时，广播通道会返回错误
                // 这是正常的行为，不需要打印错误日志
                // 只返回错误信息，让调用者决定如何处理
                Err(format!("没有活动的 SSE 接收者: {:?}", e))
            }
        }
    }
}
