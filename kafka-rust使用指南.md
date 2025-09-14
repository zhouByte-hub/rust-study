# Kafka-Rust 使用指南

## 目录

1. [项目介绍](#项目介绍)
2. [环境配置](#环境配置)
3. [安装与依赖](#安装与依赖)
4. [基本概念](#基本概念)
5. [生产者(Producer)使用](#生产者producer使用)
6. [消费者(Consumer)使用](#消费者consumer使用)
7. [高级功能](#高级功能)
8. [错误处理](#错误处理)
9. [最佳实践](#最佳实践)
10. [示例项目](#示例项目)

## 项目介绍

Kafka-Rust 是一个用 Rust 语言编写的 Apache Kafka 客户端库。它提供了与 Kafka 集群进行交互的高级 API，包括消费者（Consumer）和生产者（Producer）接口。该项目的目标是为 Rust 开发者提供一个高效、可靠的 Kafka 客户端库。

### 主要特性

- **纯 Rust 实现**：不依赖外部 C 库，提供更好的安全性和性能
- **高级 API**：提供简单易用的消费者和生产者接口
- **版本兼容性**：支持 Kafka 0.8.2 到 3.1.0 版本
- **异步支持**：支持 Rust 的异步编程模型
- **消费者组支持**：支持消费者组管理和偏移量提交

### 项目地址

- GitHub: https://github.com/kafka-rust/kafka-rust
- crates.io: https://crates.io/crates/kafka

## 环境配置

### 系统要求

- **操作系统**：Linux、macOS、Windows
- **Rust 版本**：1.56.0 或更高版本
- **Kafka 版本**：0.8.2 到 3.1.0

### 安装 Rust

如果你还没有安装 Rust，可以通过以下命令安装：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

安装完成后，确保 Rust 工具链已更新到最新版本：

```bash
rustup update
```

### 安装 Kafka

你需要在本地或远程服务器上安装 Kafka 服务器。以下是安装 Kafka 的步骤：

1. 下载 Kafka 发行版：

```bash
wget https://downloads.apache.org/kafka/3.1.0/kafka_2.13-3.1.0.tgz
```

2. 解压 Kafka 压缩包：

```bash
tar -xzf kafka_2.13-3.1.0.tgz
cd kafka_2.13-3.1.0
```

3. 启动 Kafka 服务：

```bash
# 启动 ZooKeeper
bin/zookeeper-server-start.sh config/zookeeper.properties

# 在另一个终端启动 Kafka 服务器
bin/kafka-server-start.sh config/server.properties
```

## 安装与依赖

### 添加依赖

在你的 Rust 项目中，打开 `Cargo.toml` 文件，在 `[dependencies]` 部分添加 kafka-rust 依赖：

```toml
[dependencies]
kafka = "0.10"
```

如果你需要使用异步功能，可以添加 tokio：

```toml
[dependencies]
kafka = "0.10"
tokio = { version = "1.0", features = ["full"] }
```

### 验证安装

创建一个简单的 Rust 程序来验证安装：

```rust
// main.rs
fn main() {
    println!("Kafka-Rust 安装成功!");
}
```

运行程序：

```bash
cargo run
```

## 基本概念

### Kafka 架构

- **Producer**: 消息生产者，向 Kafka broker 发送消息的客户端
- **Consumer**: 消息消费者，从 Kafka broker 接收消息的客户端
- **Consumer Group**: 消费者组，由多个 consumer 组成，共同消费一个主题
- **Broker**: 一台 Kafka 服务器就是一个 broker
- **Topic**: 主题，可以理解为一个队列，生产者和消费者面向的都是一个 topic
- **Partition**: 分区，为了实现扩展性，一个 topic 可以分为多个 partition
- **Replica**: 副本，一个 topic 的每个分区都有若干个副本
- **Leader**: 每个分区多个副本的"主"，生产者发送数据的对象
- **Follower**: 每个分区多个副本中的"从"，实时从 Leader 中同步数据

### Kafka-Rust 模块

Kafka-Rust 库主要包含以下模块：

- `kafka::client`: Kafka 客户端核心功能
- `kafka::producer`: 生产者相关功能
- `kafka::consumer`: 消费者相关功能
- `kafka::error`: 错误处理
- `kafka::utils`: 工具函数

## 生产者(Producer)使用

### 基本生产者

下面是一个简单的生产者示例，向 Kafka 主题发布消息：

```rust
use kafka::producer::{Producer, Record};
use kafka::client::KafkaClient;
use std::time::Duration;

fn main() {
    // 创建 Kafka 客户端
    let mut client = KafkaClient::new(vec!("localhost:9092"));
    
    // 设置客户端配置
    client.set_compression(Some(kafka::client::Compression::SNAPPY));
    client.set_ack_timeout(Duration::from_secs(1));
    client.set_retries(3);
    
    // 创建生产者
    let mut producer = Producer::from_client(client).with_ack_timeout(Duration::from_secs(1)).create();
    
    // 创建消息记录
    let record = Record::from_value("my-topic", "Hello, Kafka!");
    
    // 发送消息
    match producer.send(&record) {
        Ok(_) => println!("消息发送成功"),
        Err(e) => println!("消息发送失败: {}", e),
    }
}
```

### 带键的消息发送

```rust
use kafka::producer::{Producer, Record};
use kafka::client::KafkaClient;

fn main() {
    let mut client = KafkaClient::new(vec!("localhost:9092"));
    let mut producer = Producer::from_client(client).create();
    
    // 创建带键的消息记录
    let record = Record::from_key_value("my-topic", "key-1", "Hello with key!");
    
    match producer.send(&record) {
        Ok(_) => println!("带键的消息发送成功"),
        Err(e) => println!("消息发送失败: {}", e),
    }
}
```

### 批量发送消息

```rust
use kafka::producer::{Producer, Record};
use kafka::client::KafkaClient;

fn main() {
    let mut client = KafkaClient::new(vec!("localhost:9092"));
    let mut producer = Producer::from_client(client).create();
    
    // 创建多个消息记录
    let records = vec!(
        Record::from_value("my-topic", "Message 1"),
        Record::from_value("my-topic", "Message 2"),
        Record::from_value("my-topic", "Message 3"),
    );
    
    // 批量发送消息
    for record in records {
        match producer.send(&record) {
            Ok(_) => println!("消息发送成功"),
            Err(e) => println!("消息发送失败: {}", e),
        }
    }
}
```

### 异步生产者

```rust
use kafka::producer::{Producer, Record};
use kafka::client::KafkaClient;
use std::thread;
use std::time::Duration;

fn main() {
    let mut client = KafkaClient::new(vec!("localhost:9092"));
    let mut producer = Producer::from_client(client).create();
    
    // 创建消息记录
    let record = Record::from_value("my-topic", "Async message");
    
    // 异步发送消息
    let future = producer.send_async(record);
    
    // 在另一个线程中处理发送结果
    thread::spawn(move || {
        match future.wait() {
            Ok(_) => println!("异步消息发送成功"),
            Err(e) => println!("异步消息发送失败: {}", e),
        }
    });
    
    // 主线程继续执行其他任务
    println!("主线程继续执行...");
    thread::sleep(Duration::from_secs(1));
}
```

### 自定义分区

```rust
use kafka::producer::{Producer, Record};
use kafka::client::KafkaClient;
use kafka::producer::Partitioner;

// 自定义分区器
struct CustomPartitioner;

impl Partitioner for CustomPartitioner {
    fn partition(&self, topic: &str, key: Option<&[u8]>, partitions: i32) -> i32 {
        // 自定义分区逻辑，这里简单使用键的哈希值
        match key {
            Some(k) => {
                let hash = k.iter().fold(0, |acc, &x| acc.wrapping_add(x as i32));
                hash.abs() % partitions
            },
            None => 0, // 如果没有键，默认使用分区 0
        }
    }
}

fn main() {
    let mut client = KafkaClient::new(vec!("localhost:9092"));
    let mut producer = Producer::from_client(client)
        .with_partitioner(CustomPartitioner)
        .create();
    
    let record = Record::from_key_value("my-topic", "custom-key", "Message with custom partition");
    
    match producer.send(&record) {
        Ok(_) => println!("自定义分区消息发送成功"),
        Err(e) => println!("消息发送失败: {}", e),
    }
}
```

## 消费者(Consumer)使用

### 基本消费者

下面是一个简单的消费者示例，从 Kafka 主题接收消息：

```rust
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::client::KafkaClient;

fn main() {
    // 创建 Kafka 客户端
    let mut client = KafkaClient::new(vec!("localhost:9092"));
    
    // 创建消费者
    let mut consumer = Consumer::from_client(client)
        .with_group("my-group")
        .with_topic_partitions("my-topic", &[0])
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()
        .unwrap();
    
    // 消费消息
    loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                println!("接收到消息: {:?}", m);
            }
            consumer.consume_messageset(ms);
        }
        consumer.commit_consumed();
    }
}
```

### 从特定偏移量开始消费

```rust
use kafka::consumer::{Consumer, FetchOffset};
use kafka::client::KafkaClient;

fn main() {
    let mut client = KafkaClient::new(vec!("localhost:9092"));
    let mut consumer = Consumer::from_client(client)
        .with_topic_partitions("my-topic", &[0])
        .with_fallback_offset(FetchOffset::Latest)
        .create()
        .unwrap();
    
    // 从特定偏移量开始消费
    consumer.seek_partitions("my-topic", &[0], &[10]).unwrap();
    
    // 消费消息
    loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                println!("接收到消息: {:?}", m);
            }
            consumer.consume_messageset(ms);
        }
        consumer.commit_consumed();
    }
}
```

### 多分区消费

```rust
use kafka::consumer::{Consumer, FetchOffset};
use kafka::client::KafkaClient;

fn main() {
    let mut client = KafkaClient::new(vec!("localhost:9092"));
    
    // 消费多个分区
    let partitions = vec!(0, 1, 2);
    let mut consumer = Consumer::from_client(client)
        .with_topic_partitions("my-topic", &partitions)
        .with_fallback_offset(FetchOffset::Earliest)
        .create()
        .unwrap();
    
    // 消费消息
    loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                println!("分区 {}: 接收到消息: {:?}", m.partition(), m);
            }
            consumer.consume_messageset(ms);
        }
        consumer.commit_consumed();
    }
}
```

### 消费者组

```rust
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::client::KafkaClient;

fn main() {
    let mut client = KafkaClient::new(vec!("localhost:9092"));
    
    // 创建消费者组
    let mut consumer = Consumer::from_client(client)
        .with_group("my-consumer-group")
        .with_topic("my-topic")
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()
        .unwrap();
    
    // 消费消息
    loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                println!("消费者组接收到消息: {:?}", m);
            }
            consumer.consume_messageset(ms);
        }
        consumer.commit_consumed();
    }
}
```

### 手动提交偏移量

```rust
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::client::KafkaClient;

fn main() {
    let mut client = KafkaClient::new(vec!("localhost:9092"));
    let mut consumer = Consumer::from_client(client)
        .with_group("my-group")
        .with_topic("my-topic")
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()
        .unwrap();
    
    // 消费消息，但不自动提交偏移量
    let mut message_count = 0;
    loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                println!("接收到消息: {:?}", m);
                message_count += 1;
                
                // 处理消息...
                
                // 每处理 10 条消息手动提交一次偏移量
                if message_count % 10 == 0 {
                    consumer.commit_consumed();
                    println!("已提交偏移量");
                }
            }
            consumer.consume_messageset(ms);
        }
    }
}
```

## 高级功能

### 压缩配置

```rust
use kafka::producer::{Producer, Record};
use kafka::client::{KafkaClient, Compression};

fn main() {
    let mut client = KafkaClient::new(vec!("localhost:9092"));
    
    // 设置压缩类型
    client.set_compression(Some(Compression::SNAPPY));
    
    let mut producer = Producer::from_client(client).create();
    let record = Record::from_value("my-topic", "Compressed message");
    
    match producer.send(&record) {
        Ok(_) => println!("压缩消息发送成功"),
        Err(e) => println!("消息发送失败: {}", e),
    }
}
```

### 安全连接 (SSL/SASL)

```rust
use kafka::client::{KafkaClient, SecurityConfig};
use kafka::producer::Producer;

fn main() {
    // 配置安全连接
    let security_config = SecurityConfig::new()
        .with_ssl_ca_file("/path/to/ca.crt")
        .with_ssl_cert_file("/path/to/client.crt")
        .with_ssl_key_file("/path/to/client.key")
        .with_sasl_mechanism("PLAIN")
        .with_sasl_username("username")
        .with_sasl_password("password");
    
    // 创建带安全配置的客户端
    let mut client = KafkaClient::new_secure(vec!("localhost:9092"), security_config);
    let mut producer = Producer::from_client(client).create();
    
    // 发送消息
    let record = kafka::producer::Record::from_value("secure-topic", "Secure message");
    
    match producer.send(&record) {
        Ok(_) => println!("安全消息发送成功"),
        Err(e) => println!("消息发送失败: {}", e),
    }
}
```

### 自定义序列化

```rust
use kafka::producer::{Producer, Record};
use kafka::client::KafkaClient;
use serde::{Serialize, Deserialize};
use serde_json;

// 定义自定义数据结构
#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
}

// 序列化函数
fn serialize_user(user: &User) -> Vec<u8> {
    serde_json::to_vec(user).unwrap()
}

// 反序列化函数
fn deserialize_user(bytes: &[u8]) -> User {
    serde_json::from_slice(bytes).unwrap()
}

fn main() {
    let mut client = KafkaClient::new(vec!("localhost:9092"));
    let mut producer = Producer::from_client(client).create();
    
    // 创建用户对象
    let user = User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };
    
    // 序列化并发送
    let serialized = serialize_user(&user);
    let record = Record::from_value("users-topic", serialized);
    
    match producer.send(&record) {
        Ok(_) => println!("用户数据发送成功"),
        Err(e) => println!("消息发送失败: {}", e),
    }
}
```

## 错误处理

### 基本错误处理

```rust
use kafka::producer::{Producer, Record};
use kafka::client::KafkaClient;
use kafka::error::Error as KafkaError;

fn main() {
    let mut client = KafkaClient::new(vec!("localhost:9092"));
    let mut producer = Producer::from_client(client).create();
    
    let record = Record::from_value("my-topic", "Test message");
    
    match producer.send(&record) {
        Ok(_) => println!("消息发送成功"),
        Err(KafkaError::IO(e)) => println!("IO 错误: {}", e),
        Err(KafkaError::NoBrokers) => println!("没有可用的 broker"),
        Err(KafkaError::PartitionNotFound) => println!("分区未找到"),
        Err(e) => println!("其他错误: {}", e),
    }
}
```

### 重试机制

```rust
use kafka::producer::{Producer, Record};
use kafka::client::KafkaClient;
use std::thread;
use std::time::Duration;

fn send_with_retry(producer: &mut Producer, record: &Record, max_retries: usize) -> Result<(), String> {
    let mut retries = 0;
    
    loop {
        match producer.send(record) {
            Ok(_) => return Ok(()),
            Err(e) => {
                retries += 1;
                if retries >= max_retries {
                    return Err(format!("发送失败，已达到最大重试次数: {}", e));
                }
                println!("发送失败，重试 {}/{}: {}", retries, max_retries, e);
                thread::sleep(Duration::from_secs(1));
            }
        }
    }
}

fn main() {
    let mut client = KafkaClient::new(vec!("localhost:9092"));
    let mut producer = Producer::from_client(client).create();
    
    let record = Record::from_value("my-topic", "Retry message");
    
    match send_with_retry(&mut producer, &record, 3) {
        Ok(_) => println!("消息发送成功"),
        Err(e) => println!("消息发送失败: {}", e),
    }
}
```

## 最佳实践

### 生产者最佳实践

1. **批量发送**：尽可能批量发送消息以提高吞吐量
2. **压缩**：使用压缩减少网络传输量
3. **重试机制**：实现适当的重试机制处理临时故障
4. **异步发送**：对于高吞吐量场景，使用异步发送
5. **确认机制**：根据可靠性需求选择适当的确认级别

```rust
use kafka::producer::{Producer, Record};
use kafka::client::{KafkaClient, Compression};
use std::time::Duration;

fn main() {
    let mut client = KafkaClient::new(vec!("localhost:9092"));
    
    // 配置生产者
    client.set_compression(Some(Compression::SNAPPY));
    client.set_ack_timeout(Duration::from_secs(1));
    client.set_retries(3);
    
    let mut producer = Producer::from_client(client)
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(1)  // 等待 leader 确认
        .create();
    
    // 批量发送消息
    let messages = vec!(
        "Message 1",
        "Message 2",
        "Message 3",
        "Message 4",
        "Message 5",
    );
    
    for msg in messages {
        let record = Record::from_value("my-topic", msg);
        
        // 使用重试机制发送
        let mut retries = 0;
        let max_retries = 3;
        
        loop {
            match producer.send(&record) {
                Ok(_) => {
                    println!("消息发送成功: {}", msg);
                    break;
                },
                Err(e) => {
                    retries += 1;
                    if retries >= max_retries {
                        println!("消息发送失败，已达到最大重试次数: {}", e);
                        break;
                    }
                    println!("消息发送失败，重试 {}/{}: {}", retries, max_retries, e);
                    std::thread::sleep(Duration::from_millis(100));
                }
            }
        }
    }
}
```

### 消费者最佳实践

1. **消费者组**：使用消费者组实现负载均衡和故障恢复
2. **偏移量管理**：根据业务需求选择适当的偏移量提交策略
3. **错误处理**：实现健壮的错误处理机制
4. **资源管理**：确保正确关闭消费者，释放资源
5. **性能优化**：调整 poll 间隔和批量大小以优化性能

```rust
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::client::KafkaClient;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    
    // 设置 Ctrl+C 处理
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        println!("收到终止信号，正在关闭消费者...");
    }).unwrap();
    
    let mut client = KafkaClient::new(vec!("localhost:9092"));
    let mut consumer = Consumer::from_client(client)
        .with_group("my-consumer-group")
        .with_topic("my-topic")
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()
        .unwrap();
    
    println!("消费者启动，开始消费消息...");
    
    // 消费消息循环
    while running.load(Ordering::SeqCst) {
        match consumer.poll(Duration::from_millis(100)) {
            Ok(messages) => {
                for ms in messages.iter() {
                    for m in ms.messages() {
                        match process_message(m) {
                            Ok(_) => {
                                println!("成功处理消息: {:?}", m);
                            },
                            Err(e) => {
                                println!("处理消息失败: {}, 跳过此消息", e);
                                // 可以选择将失败的消息记录到死信队列
                            }
                        }
                    }
                    consumer.consume_messageset(ms);
                }
                
                // 定期提交偏移量
                consumer.commit_consumed();
            },
            Err(e) => {
                println!("拉取消息失败: {}", e);
                // 短暂等待后重试
                std::thread::sleep(Duration::from_secs(1));
            }
        }
    }
    
    // 关闭消费者
    println!("正在提交最后的偏移量...");
    consumer.commit_consumed();
    println!("消费者已关闭");
}

// 处理消息的函数
fn process_message(message: &kafka::consumer::Message) -> Result<(), String> {
    // 这里实现具体的消息处理逻辑
    let payload = std::str::from_utf8(message.value())
        .map_err(|e| format!("无效的 UTF-8: {}", e))?;
    
    println!("处理消息: {}", payload);
    
    // 模拟处理过程
    if payload.contains("error") {
        return Err("模拟处理错误".to_string());
    }
    
    Ok(())
}
```

## 示例项目

### 完整的生产者和消费者示例

下面是一个完整的示例项目，包含生产者和消费者两个部分。

#### 项目结构

```
kafka-rust-example/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── producer.rs
│   └── consumer.rs
```

#### Cargo.toml

```toml
[package]
name = "kafka-rust-example"
version = "0.1.0"
edition = "2021"

[dependencies]
kafka = "0.10"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
ctrlc = "3.2"
```

#### producer.rs

```rust
use kafka::producer::{Producer, Record};
use kafka::client::{KafkaClient, Compression};
use std::time::Duration;
use serde::{Serialize, Deserialize};
use std::thread;
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug)]
struct SensorData {
    sensor_id: String,
    timestamp: u64,
    temperature: f32,
    humidity: f32,
}

pub fn run_producer(brokers: Vec<String>, topic: String) {
    let mut client = KafkaClient::new(brokers);
    
    // 配置生产者
    client.set_compression(Some(Compression::SNAPPY));
    client.set_ack_timeout(Duration::from_secs(1));
    client.set_retries(3);
    
    let mut producer = Producer::from_client(client)
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(1)
        .create();
    
    println!("生产者启动，开始发送消息到主题: {}", topic);
    
    // 模拟传感器数据
    let sensor_ids = vec!("sensor-1", "sensor-2", "sensor-3");
    
    loop {
        for sensor_id in &sensor_ids {
            // 生成随机传感器数据
            let data = SensorData {
                sensor_id: sensor_id.to_string(),
                timestamp: SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                temperature: 20.0 + rand::random::<f32>() * 15.0,
                humidity: 30.0 + rand::random::<f32>() * 40.0,
            };
            
            // 序列化数据
            let payload = match serde_json::to_vec(&data) {
                Ok(p) => p,
                Err(e) => {
                    println!("序列化失败: {}", e);
                    continue;
                }
            };
            
            // 创建消息记录
            let record = Record::from_key_value(&topic, sensor_id.as_bytes(), payload);
            
            // 发送消息
            match producer.send(&record) {
                Ok(_) => {
                    println!("发送成功: {:?}", data);
                },
                Err(e) => {
                    println!("发送失败: {}", e);
                }
            }
        }
        
        // 等待一段时间再发送下一批数据
        thread::sleep(Duration::from_secs(1));
    }
}
```

#### consumer.rs

```rust
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::client::KafkaClient;
use serde::{Deserialize};
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use serde_json;

#[derive(Deserialize, Debug)]
struct SensorData {
    sensor_id: String,
    timestamp: u64,
    temperature: f32,
    humidity: f32,
}

pub fn run_consumer(brokers: Vec<String>, topic: String, group_id: String) {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    
    // 设置 Ctrl+C 处理
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        println!("收到终止信号，正在关闭消费者...");
    }).unwrap();
    
    let mut client = KafkaClient::new(brokers);
    let mut consumer = Consumer::from_client(client)
        .with_group(&group_id)
        .with_topic(&topic)
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()
        .unwrap();
    
    println!("消费者启动，开始消费主题 {} 的消息...", topic);
    
    // 消费消息循环
    while running.load(Ordering::SeqCst) {
        match consumer.poll(Duration::from_millis(100)) {
            Ok(messages) => {
                for ms in messages.iter() {
                    for m in ms.messages() {
                        match process_message(m) {
                            Ok(_) => {
                                // 成功处理消息
                            },
                            Err(e) => {
                                println!("处理消息失败: {}, 跳过此消息", e);
                            }
                        }
                    }
                    consumer.consume_messageset(ms);
                }
                
                // 定期提交偏移量
                consumer.commit_consumed();
            },
            Err(e) => {
                println!("拉取消息失败: {}", e);
                // 短暂等待后重试
                std::thread::sleep(Duration::from_secs(1));
            }
        }
    }
    
    // 关闭消费者
    println!("正在提交最后的偏移量...");
    consumer.commit_consumed();
    println!("消费者已关闭");
}

// 处理消息的函数
fn process_message(message: &kafka::consumer::Message) -> Result<(), String> {
    // 反序列化消息
    let data: SensorData = match serde_json::from_slice(message.value()) {
        Ok(d) => d,
        Err(e) => {
            return Err(format!("反序列化失败: {}", e));
        }
    };
    
    // 处理数据
    println!("处理传感器数据 - ID: {}, 时间: {}, 温度: {:.1}°C, 湿度: {:.1}%", 
             data.sensor_id, data.timestamp, data.temperature, data.humidity);
    
    // 这里可以添加更多的处理逻辑，例如存储到数据库、触发警报等
    
    Ok(())
}
```

#### main.rs

```rust
mod producer;
mod consumer;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("用法:");
        println!("  生产者: cargo run -- producer");
        println!("  消费者: cargo run -- consumer [group_id]");
        return;
    }
    
    let brokers = vec!("localhost:9092".to_string());
    let topic = "sensor-data".to_string();
    
    match args[1].as_str() {
        "producer" => {
            producer::run_producer(brokers, topic);
        },
        "consumer" => {
            let group_id = if args.len() > 2 {
                args[2].clone()
            } else {
                "sensor-consumer-group".to_string()
            };
            consumer::run_consumer(brokers, topic, group_id);
        },
        _ => {
            println!("无效的参数: {}", args[1]);
        }
    }
}
```

### 运行示例

1. 启动 Kafka 服务器（确保 ZooKeeper 和 Kafka 已启动）

2. 创建主题：

```bash
bin/kafka-topics.sh --create --topic sensor-data --bootstrap-server localhost:9092 --partitions 3 --replication-factor 1
```

3. 运行生产者：

```bash
cargo run -- producer
```

4. 在另一个终端运行消费者：

```bash
cargo run -- consumer
```

或者使用不同的消费者组运行多个消费者：

```bash
# 终端 1
cargo run -- consumer group-1

# 终端 2
cargo run -- consumer group-2
```

## 总结

Kafka-Rust 是一个功能强大、易于使用的 Rust Kafka 客户端库。它提供了丰富的 API 和灵活的配置选项，可以满足各种场景下的消息传递需求。

本指南涵盖了 Kafka-Rust 的基本概念、安装配置、生产者和消费者的使用方法，以及一些高级功能和最佳实践。通过示例项目，你可以快速上手并开始构建自己的 Kafka 应用。

希望这份指南能够帮助你更好地理解和使用 Kafka-Rust 库！