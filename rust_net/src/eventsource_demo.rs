/**
 * reqwest-eventsource = "0.6.0"
 *
 * 提供一个简单的包装器用于[ reqwest ]以提供事件源实现。您可以通过查看 MDN 文档了解更多关于服务器发送事件（SSE）的信息。
 */

#[cfg(test)]
mod eventsource_test {
    use std::time::Duration;

    use futures_util::StreamExt;
    use reqwest::Method;
    use reqwest::Request;
    use reqwest::RequestBuilder;
    use reqwest::header;
    use reqwest_eventsource::Event;
    use reqwest_eventsource::EventSource;
    use url::Url;

    /**
     * 使用eventsource实现一个SSE客户端，
     */

    #[tokio::test]
    async fn test() {
        let client = reqwest::ClientBuilder::new()
            .timeout(Duration::from_secs(30)) // 禁用超时
            .build()
            .unwrap();
        let mut request = Request::new(
            Method::GET,
            Url::parse("http://localhost:8080/sse2").unwrap(),
        );
        request.headers_mut().insert(
            header::ACCEPT,
            header::HeaderValue::from_static("text/event-stream"),
        );

        let mut event_source =
            EventSource::new(RequestBuilder::from_parts(client, request)).unwrap();

        while let Some(event) = event_source.next().await {
            match event {
                Ok(Event::Open) => println!("连接已打开"),
                Ok(Event::Message(msg)) => {
                    println!("收到消息: {:?}", msg);
                }
                Err(e) => println!("发生错误: {:?}", e),
            }
        }
    }
}
