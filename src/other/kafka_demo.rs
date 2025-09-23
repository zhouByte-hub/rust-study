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
            AdminClient, AdminOptions, AlterConfig, NewPartitions, NewTopic, ResourceSpecifier,
            TopicReplication,
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
     * 获取消费者水印偏移量
     */
    #[tokio::test]
    async fn future_consumer_test4() {
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

        let mut tpl = TopicPartitionList::new();
        tpl.add_partition("test", 0);

        let watermarks = consumer
            .fetch_watermarks("test", 0, Timeout::from(Duration::from_secs(5)))
            .unwrap();
        println!("watermarks: {:?}", watermarks);
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

    /**
     * 使用 Admin 创建主题
     */
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

    /**
     * 使用 Admin 删除主题
     */
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

    /**
     * Watermark（水位线）是一个非常核心的概念，主要用于描述 日志（Log）中消息的边界位置。
     * 水位线分为 low watermark 和 high watermark 两种。
     * low watermark 表示消费者当前能够消费的最早的消息的偏移量，
     * high watermark 表示生产者当前能够写入的最新的消息的偏移量。
     *
     * Lag指消费者当前落后于最新消息的程度 —— 即“还有多少条消息没来得及消费
     * Lag = 分区的最新消息偏移量（High Watermark） - 消费者已消费的偏移量（Consumer Offset）
     */
    #[tokio::test]
    async fn lag_test() {
        use rdkafka::TopicPartitionList;
        use rdkafka::config::ClientConfig;
        use rdkafka::consumer::{BaseConsumer, Consumer};

        // 创建管理员客户端
        let admin_client: AdminClient<DefaultClientContext> = ClientConfig::new()
            .set("bootstrap.servers", "43.139.97.119:9092")
            .create()
            .expect("Failed to create admin client");

        // 获取主题的元数据，包括所有分区信息
        let metadata = admin_client
            .inner()
            .fetch_metadata(None, Timeout::from(Duration::from_secs(5)))
            .unwrap();

        // 获取主题的所有分区
        let topic_metadata = metadata
            .topics()
            .iter()
            .find(|topic| topic.name() == "test")
            .expect("Topic 'test' not found");

        let partitions = topic_metadata.partitions();
        println!("Topic 'test' has {} partitions", partitions.len());

        // 创建消费者客户端
        let consumer: BaseConsumer = ClientConfig::new()
            .set("bootstrap.servers", "43.139.97.119:9092")
            .set("group.id", "test_consumer_group")
            .set("enable.auto.commit", "false")
            .create()
            .expect("Failed to create consumer");

        // 创建主题分区列表
        let mut tpl = TopicPartitionList::new();
        for partition in partitions {
            tpl.add_partition("test", partition.id());
        }

        // 获取消费者在分区的已提交偏移量
        let committed_offsets = consumer
            .committed(Duration::from_secs(5))
            .unwrap_or_else(|_| panic!("Failed to fetch committed offsets"));

        // 获取消费者在分区的当前位置
        let position = consumer
            .position()
            .unwrap_or_else(|_| panic!("Failed to fetch position"));

        // 遍历所有分区，计算每个分区的滞后量
        for partition in partitions {
            let partition_id = partition.id();

            // 获取分区的最新偏移量(low watermark, high watermark)
            let watermarks = admin_client
                .inner()
                .fetch_watermarks("test", partition_id, Timeout::from(Duration::from_secs(5)))
                .unwrap_or_else(|_| {
                    panic!("Failed to fetch watermarks for partition {}", partition_id)
                });

            // 从已提交偏移量中提取当前分区的偏移量
            let consumer_offset =
                if let Some(elem) = committed_offsets.find_partition("test", partition_id) {
                    elem.offset()
                } else {
                    // 如果没有提交的偏移量，从当前位置获取
                    if let Some(elem) = position.find_partition("test", partition_id) {
                        elem.offset()
                    } else {
                        // 如果都没有，默认为0
                        rdkafka::Offset::Beginning
                    }
                };

            // 将 Offset 枚举转换为 i64
            let consumer_offset_i64 = match consumer_offset {
                rdkafka::Offset::Beginning => 0,
                rdkafka::Offset::End => watermarks.1,
                rdkafka::Offset::Offset(offset) => offset,
                rdkafka::Offset::Invalid => 0,
                rdkafka::Offset::Stored => 0,
                rdkafka::Offset::OffsetTail(offset) => offset,
            };

            // 计算滞后量：分区最新偏移量 - 消费者偏移量
            let lag = watermarks.1.saturating_sub(consumer_offset_i64);

            println!(
                "Partition {}: High Watermark = {}, Consumer Offset = {}, Lag = {}",
                partition_id, watermarks.1, consumer_offset_i64, lag
            );
        }
    }
}

#[cfg(target_os = "macos")]
#[cfg(test)]
mod transaction_test {
    use rdkafka::util::Timeout;
    use rdkafka::{
        ClientConfig,
        producer::{FutureProducer, FutureRecord, Producer},
    };
    use tokio::time::Duration;

    /**
     * 开启事务操作
     */
    #[tokio::test]
    async fn transaction_test_v1() {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", "43.139.97.119:9092")
            .set("transactional.id", "test_transactional_id")
            .set("enable.idempotence", "true")
            .create()
            .expect("Transactional producer creation failed");

        /*
           这个函数确保先前具有相同 transactional.id 的生产者发起的任何事务都能完成。任何此类先前生产者留下的未完成事务将被中止。
           一旦先前事务被锁定，这个函数将获取一个内部生产者 ID 和时间戳，这些将被用于此生产者发送的所有事务性消息。
           如果这个函数成功返回，只有在事务活动时才能向此生产者发送消息。参见 [Producer::begin_transaction]。
           这个函数可能会阻塞指定的超时时间。
        */
        producer
            .init_transactions(Timeout::from(Duration::from_secs(5)))
            .unwrap();

        // 开始事务
        producer.begin_transaction().unwrap();

        let message = vec![("name", "zhangsan"), ("address", "beijing")];
        // 发送消息
        for msg in message {
            let record = FutureRecord::to("test").key(msg.0).payload(msg.1);
            producer
                .send(record, Timeout::from(Duration::from_secs(5)))
                .await
                .unwrap();
        }
        // 提交事务
        producer
            .commit_transaction(Timeout::from(Duration::from_secs(5)))
            .unwrap();
    }
}

#[cfg(target_os = "macos")]
#[cfg(test)]
mod context {
    use rdkafka::ClientConfig;
    use rdkafka::producer::FutureProducer;
    use rdkafka::producer::FutureRecord;
    use rdkafka::producer::Producer;
    use rdkafka::util::Timeout;
    use rdkafka::{ClientContext, producer::ProducerContext};
    use tokio::time::Duration;

    struct LogginProducerContext;

    impl ClientContext for LogginProducerContext {}

    impl ProducerContext for LogginProducerContext {
        type DeliveryOpaque = ();

        fn delivery(
            &self,
            delivery_result: &rdkafka::message::DeliveryResult<'_>,
            _delivery_opaque: Self::DeliveryOpaque,
        ) {
            match delivery_result {
                Ok(msg) => println!("Message delivered: {:?}", msg),
                Err((err, msg)) => println!("Delivery failed: {:?}, message: {:?}", err, msg),
            }
        }

        fn get_custom_partitioner(&self) -> Option<&rdkafka::producer::NoCustomPartitioner> {
            None
        }
    }

    /**
     * isolation.level： 是一个消费者（Consumer）配置参数，用于控制消费者在读取消息时如何处理未提交的事务性消息。这个配置主要与 Kafka 的事务性写入（Transactional Writes）和读已提交（Read Committed）语义相关。
     *      1、read_uncommitted：消费者可以读取所有消息，包括：已提交的事务消息、未提交的事务消息、非事务性消息；消费者可能会看到最终会被回滚（abort）的消息，可能导致脏读。
     *      2、read_committed：消费者只能读取已提交的事务消息，不能读取未提交的事务消息。这确保了消费者只能看到最终被提交的消息，避免了脏读问题。
     *
     * enable.idempotenceP：是 Apache Kafka 生产者（Producer）的一个重要配置属性，用于启用幂等性生产者（Idempotent Producer）功能。
     * 当 enable.idempotence=true 时，Kafka 会保证单个生产者实例在重试发送消息的过程中，不会向主题（Topic）中重复写入同一条消息。
     * 换句话说，即使由于网络问题、Broker 故障等原因导致发送失败并触发重试，Kafka 也能确保每条消息在日志中恰好出现一次（Exactly Once Semantics, EOS 的一部分）。
     */
    #[tokio::test]
    async fn test_producer_context() {
        let producer: FutureProducer<LogginProducerContext> = ClientConfig::new()
            .set("bootstrap.servers", "43.139.97.119:9092")
            .set("message.timeout.ms", "30000")
            .set("acks", "1")
            .set("retries", "3")
            .set("request.timeout.ms", "30000")
            .set("transactional.id", "test_transactional_id")
            .set("enable.idempotence", "true")
            .set("acks", "all")
            .create_with_context(LogginProducerContext)
            .expect("Failed to create producer");

        producer
            .init_transactions(Timeout::from(Duration::from_secs(5)))
            .unwrap();

        producer.begin_transaction().unwrap();

        let message = vec![("name", "zhangsan"), ("address", "beijing")];
        // 发送消息
        for msg in message {
            let record = FutureRecord::to("test").key(msg.0).payload(msg.1);
            producer
                .send(record, Timeout::from(Duration::from_secs(5)))
                .await
                .unwrap();
        }
        // 提交事务
        producer
            .commit_transaction(Timeout::from(Duration::from_secs(5)))
            .unwrap();
    }
}
