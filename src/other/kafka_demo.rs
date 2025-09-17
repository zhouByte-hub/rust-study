/**
 *  rdkafka = "0.38.0"
 *  rdKafka提供了以下组件：
 *      1、FutureProducer：异步生产者
 *      2、BaseProducer：同步生产者
 *      3、StreamConsumer：基于流的消费者
 *      4、BaseConsumer：同步消费者
 *      5、AdminClient：管理员客户端，用于管理 Kafka 集群
 */

#[cfg(target_os = "macos")]
#[cfg(test)]
mod kafka_test {
    use rdkafka::{
        consumer::{CommitMode, Consumer, StreamConsumer}, message::{Header, OwnedHeaders}, producer::{FutureProducer, FutureRecord}, ClientConfig, Offset, TopicPartitionList
    };
    use tokio::time::Duration;
    use tokio_stream::StreamExt;

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

    #[tokio::test]
    async fn future_consumer_test() {
        // 创建Kafka消费者配置
        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", "future_consumer_test")  // 使用唯一的消费者组ID
            .set("bootstrap.servers", "43.139.97.119:9092")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "false")
            // 设置从最早的消息开始消费
            .set("auto.offset.reset", "earliest")
            // 禁用分区结束信号
            .set("enable.partition.eof", "false")
            .create()
            .expect("Failed to create consumer");
        
        // 订阅主题
        consumer.subscribe(&["test"]).expect("订阅失败");
        println!("消费者已启动，等待消息...");
        
        while let Some(message) = consumer.stream().next().await {
            match message {
                Ok(message) => {
                    println!("Received message: {:?}", message);
                    // 手动提交偏移量
                    consumer.commit_message(&message, CommitMode::Async).expect("提交偏移量失败");
                }
                Err(e) => {
                    eprintln!("Error receiving message: {}", e);
                }
            }
        }
   
    }


    /**
     * 从特定分区和偏移量进行消费
     */
    #[tokio::test]
    async fn future_consumer_test2(){
        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", "future_consumer_test")  // 使用唯一的消费者组ID
            .set("bootstrap.servers", "43.139.97.119:9092")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "false")
            // 设置从最早的消息开始消费
            .set("auto.offset.reset", "earliest")
            // 禁用分区结束信号
            .set("enable.partition.eof", "false")
            .create()
            .expect("Failed to create consumer");

        // 设置分区和偏移量
        let mut topic_partition_list = TopicPartitionList::new();
        topic_partition_list.add_partition_offset("test", 0, Offset::Beginning).unwrap();

        consumer.assign(&topic_partition_list).unwrap();

        while let Some(message) = consumer.stream().next().await {
            match message {
                Ok(message) => {
                    println!("Received message: {:?}", message);
                    // 手动提交偏移量
                    consumer.commit_message(&message, CommitMode::Async).expect("提交偏移量失败");
                }
                Err(e) => {
                    eprintln!("Error receiving message: {}", e);
                }
            }
        }
    }


    /**
     * 获取消费者位置
     */
    #[tokio::test]
    async fn future_consumer_test3() {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", "future_consumer_test")  // 使用唯一的消费者组ID
            .set("bootstrap.servers", "43.139.97.119:9092")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "false")
            // 设置从最早的消息开始消费
            .set("auto.offset.reset", "earliest")
            // 禁用分区结束信号
            .set("enable.partition.eof", "false")
            .create()
            .expect("Failed to create consumer");

        // 获取消费者在每个分区的当前偏移量
        let position = consumer.position().unwrap();
        for element in position.elements() {
            println!("partition: {}, offset: {:?}", element.partition(), element.offset());
        }

    }
}
