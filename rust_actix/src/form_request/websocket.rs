use actix_web::{rt, Error, HttpRequest, HttpResponse, get, web};
use actix_ws::AggregatedMessage;
use futures_util::StreamExt;

/**
 * actix-ws = "0.3.0"
 * 
 * Actix Web 通过 actix-ws crate 支持 WebSocket 的高级接口。
 * 使用这个 crate，可以将请求的 Payload 流转换为 ws::Messages 流，然后在创建的异步任务中对其做出响应。
 */

#[get("/ws")]
pub async fn websocket_test(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    // 调用actix_ws::handle函数处理WebSocket握手，返回三个值：res是握手成功的HTTP响应，session是WebSocket会话对象，stream是消息流。
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        // 对流进行配置，启用消息聚合功能，将分片的WebSocket消息自动重组。
        .aggregate_continuations()
        // 设置最大聚合大小为1MB（2的20次方字节），防止内存过度使用。
        .max_continuation_size(2_usize.pow(20));

    rt::spawn(async move {
        while let Some(msg) = stream.next().await {
            match msg {
                // 如果接收到文本消息，直接将相同的文本消息发送回客户端（echo功能）
                Ok(AggregatedMessage::Text(text)) => session.text(text).await.unwrap(),

                // 如果接收到二进制消息，直接将相同的二进制数据发送回客户端。
                Ok(AggregatedMessage::Binary(bin)) => session.binary(bin).await.unwrap(),

                // 如果接收到Ping控制帧，自动回复Pong帧，保持连接活跃。
                Ok(AggregatedMessage::Ping(msg)) => session.ping(&msg).await.unwrap(),
                _ => {}
            }
        }
    });
    Ok(res)
}