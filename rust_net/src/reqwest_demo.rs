/**
 * reqwest = "0.12.23"
 *
 *  Rust 生态系统中一个非常流行且功能强大的 HTTP 客户端库。它让你的 Rust 程序能够轻松地发送 HTTP 请求（如 GET, POST, PUT, DELETE 等）并处理响应。
 *  它建立在 tokio 异步运行时之上，提供了现代、简洁且高效的 API，所以一般与tokio一起使用。
 *
 *  features:
 *      1、default-tls：使用 native-tls crate 作为 TLS (HTTPS) 后端。
 *      2、rustls-tls：使用 rustls crate 作为 TLS 后端。
 *      3、gzip：支持自动解压 gzip 压缩的响应体。
 *      4、brotli：支持自动解压 br (Brotli) 压缩的响应体。
 *      5、json：启用 .json() 和 .send().json() 方法，用于方便地序列化/反序列化 JSON 数据，依赖serde with derive feature, serde_json
 *      6、cookies：启用 Cookie 存储和管理功能，依赖cookie, cookie-store
 *      7、stream：流式处理
 */

#[cfg(test)]
mod reqwest_demo {
    use futures_util::stream::StreamExt;
    use std::collections::HashMap;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_1() {
        let response = reqwest::get("http://www.baidu.com").await.unwrap();
        if response.status() == reqwest::StatusCode::OK {
            let body = response.text().await.unwrap();
            println!("{}", body);
        }
    }

    #[tokio::test]
    async fn test_2() {
        let client = reqwest::Client::new();

        let mut body = HashMap::new();
        body.insert("name", "zhangsan");
        body.insert("id", "12138");

        /*  发送请求体
               1、body
               2、json
               3、from：表单
        */
        let response = client
            .post("http:://localhost:8080")
            .json(&body)
            .send()
            .await
            .unwrap();

        if response.status() == reqwest::StatusCode::OK {
            let body = response.text().await.unwrap();
            println!("{}", body);
        }
    }

    #[tokio::test]
    async fn test_3() {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(60)) // 总超时时长
            .read_timeout(Duration::from_secs(10)) // 读取超时
            .connect_timeout(Duration::from_secs(10)) // 连接超时
            .gzip(true)
            .user_agent("my-rust-app/1.0")
            .build()
            .unwrap();

        /*  响应体读取方式
               1、text：读取文本响应体                 异步        将数据全部加载进内存     适合文本内容：JSON、HTML和日志等。
               2、bytes：读取原始字节响应体            异步        将数据全部加载进内存      二进制数据（图片、音频等）
               3、json：将响应体解析为 JSON 格式
               4、stream：流式处理响应体               异步        流式逐块接收            大文件、流式日志、SSE、视频流等
               5、chunked：分块读取响应体
        */

        let response = client.get("http://www.baidu.com").send().await.unwrap();
        // text
        // if response.status() == reqwest::StatusCode::OK {
        //     let body = response.text().await.unwrap();
        //     println!("{}", body);
        // }

        // stream
        let mut stream = response.bytes_stream();
        let mut list = Vec::new();
        while let Some(item) = stream.next().await {
            let chunk = item.unwrap();
            list.push(chunk);
        }
        println!("{:?}", list);
    }
}
