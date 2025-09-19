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
        ClientConfig, Offset, TopicPartitionList,
        admin::{
            AdminClient, AdminOptions, AlterConfig, NewPartitions, NewTopic,
            ResourceSpecifier, TopicReplication,
        },
        client::DefaultClientContext,
        consumer::{CommitMode, Consumer, StreamConsumer},
        message::{Header, OwnedHeaders},
        producer::{FutureProducer, FutureRecord},
        util::Timeout,
    };
    use std::collections::HashMap;
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
            .set("group.id", "future_consumer_test") // 使用唯一的消费者组ID
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
                    consumer
                        .commit_message(&message, CommitMode::Async)
                        .expect("提交偏移量失败");
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
    async fn future_consumer_test2() {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", "future_consumer_test") // 使用唯一的消费者组ID
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
        topic_partition_list
            .add_partition_offset("test", 0, Offset::Beginning)
            .unwrap();

        consumer.assign(&topic_partition_list).unwrap();

        while let Some(message) = consumer.stream().next().await {
            match message {
                Ok(message) => {
                    println!("Received message: {:?}", message);
                    // 手动提交偏移量
                    consumer
                        .commit_message(&message, CommitMode::Async)
                        .expect("提交偏移量失败");
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
            .set("group.id", "future_consumer_test") // 使用唯一的消费者组ID
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
            println!(
                "partition: {}, offset: {:?}",
                element.partition(),
                element.offset()
            );
        }
    }

    /**
     * 创建主题
     */
    #[tokio::test]
    async fn admin_test1() {
        let admin_client: AdminClient<DefaultClientContext> = ClientConfig::new()
            .set("bootstrap.servers", "43.139.97.119:9092")
            .create()
            .expect("Failed to create admin client");

        let topic = NewTopic::new("topic_test", 1, TopicReplication::Fixed(1));

        // topic.set("key", "value"); // 设置主题的配置信息

        let options = AdminOptions::new().request_timeout(Some(Duration::from_secs(30)));
        let result = admin_client
            .create_topics(&[topic], &options)
            .await
            .unwrap();
        for item in result {
            match item {
                Ok(topic) => {
                    println!("topic: {:?}", topic);
                }
                Err(e) => {
                    eprintln!("Failed to create topic: {:?}", e);
                }
            }
        }
    }

    #[tokio::test]
    async fn admin_test2() {
        let admin_client: AdminClient<DefaultClientContext> = ClientConfig::new()
            .set("bootstrap.servers", "43.139.97.119:9092")
            .create()
            .expect("Failed to create admin client");

        let topic_name = "test";
        let mut configs = HashMap::new();
        // 设置日志清理策略为删除和压缩
        configs.insert("cleanup.policy".to_string(), "delete,compact".to_string());
        // 设置消息保留时间为3天
        configs.insert("retention.ms".to_string(), "259200000".to_string());
        // 设置单个日志段的最大大小为512MB
        configs.insert("segment.bytes".to_string(), "536870912".to_string());
        // 设置单个消息的最大大小为5MB
        configs.insert("max.message.bytes".to_string(), "5242880".to_string());
        // 设置消息时间戳类型为日志追加时间
        configs.insert(
            "message.timestamp.type".to_string(),
            "LogAppendTime".to_string(),
        );
        // 设置压缩类型为lz4
        configs.insert("compression.type".to_string(), "lz4".to_string());
        // 设置必须成功写入的最小副本数为2
        configs.insert("min.insync.replicas".to_string(), "2".to_string());
        let mut new_topic = NewTopic::new(topic_name, 6, TopicReplication::Fixed(3));
        // 应用配置
        for (key, value) in &configs {
            new_topic = new_topic.set(key, value);
        }
        let options = AdminOptions::new().request_timeout(Some(Duration::from_secs(10)));
        let result = admin_client.create_topics(&[new_topic], &options).await;
        match result {
            Ok(value) => {
                for item in value {
                    match item {
                        Ok(topic) => {
                            println!("topic: {:?}", topic);
                        }
                        Err(e) => {
                            eprintln!("Failed to create topic: {:?}", e);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to create topic '{}': {}", topic_name, e);
            }
        };
    }

    #[tokio::test]
    async fn admin_delete_topic() {
        let admin_client: AdminClient<DefaultClientContext> = ClientConfig::new()
            .set("bootstrap.servers", "43.139.97.119:9092")
            .create()
            .expect("Failed to create admin client");

        let opts = AdminOptions::new().request_timeout(Some(Duration::from_secs(5)));

        let result = admin_client.delete_topics(&["test"], &opts).await.unwrap();
        for item in result {
            match item {
                Ok(topic) => {
                    println!("topic: {:?}", topic);
                }
                Err(e) => {
                    eprintln!("Failed to delete topic: {:?}", e);
                }
            }
        }
    }

    /**
     * 获取所有主题
     */
    #[tokio::test]
    async fn topic_list() {
        let admin_client: AdminClient<DefaultClientContext> = ClientConfig::new()
            .set("bootstrap.servers", "43.139.97.119:9092")
            .create()
            .expect("Failed to create admin client");

        let metadata = admin_client
            .inner()
            .fetch_metadata(None, Timeout::from(Duration::from_secs(5)))
            .unwrap();
        let topic_names = metadata
            .topics()
            .iter()
            .map(|item| item.name().to_string())
            .collect::<Vec<String>>();
        println!("metadata: {:?}", topic_names);
    }

    /**
     * 更新主题配置
     */
    #[tokio::test]
    async fn update_topic_config() {
        // 创建Kafka管理员客户端
        let admin_client: AdminClient<DefaultClientContext> = ClientConfig::new()
            .set("bootstrap.servers", "43.139.97.119:9092")
            .create()
            .expect("Failed to create admin client");

        // 创建并配置AlterConfig，使用链式调用避免所有权问题
        let mut alter_config = AlterConfig::new(ResourceSpecifier::Topic("test"));

        alter_config = alter_config.set("cleanup.policy", "compact");

        // 设置管理员选项
        let options = AdminOptions::new().request_timeout(Some(Duration::from_secs(10)));

        // 执行配置更新
        admin_client
            .alter_configs(&[alter_config], &options)
            .await
            .unwrap();
    }

    /**
     * 获取主题配置列表
     */
    #[tokio::test]
    async fn config_list() {
        let admin_client: AdminClient<DefaultClientContext> = ClientConfig::new()
            .set("bootstrap.servers", "43.139.97.119:9092")
            .create()
            .expect("Failed to create admin client");

        let options = AdminOptions::new().request_timeout(Some(Duration::from_secs(10)));

        // 获取 test 主题的所有配置
        let config = admin_client
            .describe_configs(&[ResourceSpecifier::Topic("test")], &options)
            .await
            .unwrap();
        for item in config {
            match item {
                Ok(config) => {
                    for item in config.entries {
                        println!("{}: {}", item.name, item.value.unwrap());
                    }
                }
                Err(e) => {
                    eprintln!("Failed to describe config: {:?}", e);
                }
            }
        }
    }

    /**
     * 为主题创建分区
     */
    #[tokio::test]
    async fn create_partitions() {
        let admin_client: AdminClient<DefaultClientContext> = ClientConfig::new()
            .set("bootstrap.servers", "43.139.97.119:9092")
            .create()
            .expect("Failed to create admin client");

        // 新建分区可以相当于修改分区
        let options = AdminOptions::new().request_timeout(Some(Duration::from_secs(10)));

        let new_partitions = NewPartitions::new("test", 3);
        admin_client
            .create_partitions(&[new_partitions], &options)
            .await
            .unwrap();
    }

    /**
     * 获取主题的分区数
     */
    #[tokio::test]
    async fn partition_count() {
        let admin_client: AdminClient<DefaultClientContext> = ClientConfig::new()
            .set("bootstrap.servers", "43.139.97.119:9092")
            .create()
            .expect("Failed to create admin client");

        let meta = admin_client
            .inner()
            .fetch_metadata(Some("test"), Timeout::from(Duration::from_secs(5)))
            .unwrap();

        let partition_count = meta
            .topics()
            .iter()
            .filter(|item| item.name() == "test")
            .next()
            .unwrap()
            .partitions()
            .len();
        println!("partition count: {}", partition_count);
    }

    
}
