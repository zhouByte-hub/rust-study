/**
 *  rdkafka = "0.38.0"
 *  rdKafka提供了以下组件：
 *      1、FutureProducer：异步生产者
 *      2、BaseProducer：同步生产者
 *      3、StreamConsumer：基于流的消费者
 *      4、BaseConsumer：同步消费者
 *      5、AdminClient：管理员客户端，用于管理 Kafka 集群
 */

#[cfg(target_os="macos")]
#[cfg(test)]
mod kafka_test {
    use rdkafka::{
        ClientConfig,
        message::{Header, OwnedHeaders},
        producer::{FutureProducer, FutureRecord},
    };
    use tokio::time::Duration;

    #[tokio::test]
    async fn future_producer_test() {
        // 创建Kafka生产者配置
        let producer: FutureProducer = ClientConfig::new()
            // 修改：去掉http://前缀，使用标准Kafka协议
            .set("bootstrap.servers", "43.139.97.119:9092")
            // 统一超时配置为30秒
            .set("message.timeout.ms", "30000")
            // 设置acks为1，只需要leader确认，提高响应速度
            .set("acks", "1")
            // 添加重试配置
            .set("retries", "3")
            // 设置请求超时
            .set("request.timeout.ms", "30000")
            .create()
            .expect("Failed to create producer");

        // 创建消息记录
        let record = FutureRecord::to("test").payload("hello world").key("key");

        // 发送消息并处理结果
        match producer.send(record, Duration::from_secs(30)).await {
            Ok(delivery_status) => {
                println!("Message sent successfully: {:?}", delivery_status);
            }
            Err((e, _message)) => {
                eprintln!("Failed to send message: {}", e);
                panic!("Message production failed: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn future_producer_test2() {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", "43.139.97.119:9092")
            .set("message.timeout.ms", "30000")
            .set("acks", "1")
            .set("retries", "3")
            .set("request.timeout.ms", "30000")
            .create()
            .expect("Failed to create producer");

        // 创建Kafka消息头信息
        let mut headers = OwnedHeaders::new();
        // 添加消息头，参数为(键, 值)
        headers = headers.insert(Header {
            key: "name",
            value: Some("zhangsan"),
        });

        let record = FutureRecord::to("test")
            .partition(0) // 指定发送的分区
            .payload("添加消息头的消息")
            .key("key")
            .headers(headers);

        match producer.send(record, Duration::from_secs(30)).await {
            Ok(delivery_status) => {
                println!("Message sent successfully: {:?}", delivery_status);
            }
            Err((e, _message)) => {
                eprintln!("Failed to send message: {}", e);
                panic!("Message production failed: {}", e);
            }
        }
    }
}
