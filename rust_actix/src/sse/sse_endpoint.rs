use std::convert::Infallible;

use crate::sse::sender::SseSender;
use actix_web::Responder;
use actix_web::web;
use actix_web_lab::sse;
use actix_web_lab::sse::Sse;
use futures_util::StreamExt;
use tokio_stream::wrappers::BroadcastStream;
/// 创建 SSE 流端点
///
/// 该函数创建一个服务器发送事件（SSE）流，允许客户端订阅并接收实时消息
/// 它使用广播通道将消息发送给所有订阅的客户端，并在发送完所有消息后自动断开连接
///
/// # 参数
/// * `sender` - SSE 发送器的应用状态
///
/// # 返回值
/// 返回一个 SSE 响应，用于建立服务器发送事件流
pub async fn sse_stream(sender: web::Data<SseSender>) -> impl Responder {
    // 创建一个新的广播通道接收者
    let rx = sender.create_channel();

    for i in 0..10 {
        if let Err(e) = sender.send(format!("message = {}", i)) {
            eprintln!("发送消息失败: {:?}", e);
            break;
        }
    }

    // 将广播流转换为 SSE 事件流
    let sse_stream = BroadcastStream::new(rx)
        .filter_map(|msg| async move {
            match msg {
                Ok(data) => {
                    // 成功接收到数据，创建 SSE 事件
                    let send_data = sse::Data::new(data);
                    Some(sse::Event::Data(send_data))
                }
                Err(e) => {
                    // 处理广播流中的错误
                    // BroadcastStream 错误通常表示通道已关闭或没有更多发送者
                    // 在这种情况下，我们记录错误并关闭流
                    eprintln!("SSE broadcast stream error: {:?}", e);
                    None
                }
            }
        })
        .map(|event| Ok::<sse::Event, Infallible>(event)); // 将错误转换为 Infallible

    // 创建 SSE 响应，并设置保持连接的时间
    Sse::from_stream(sse_stream)
}
