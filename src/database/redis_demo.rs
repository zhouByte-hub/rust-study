/**
 * redis = {version = "0.32.5", features = ["r2d2", "tokio-comp", "json"]}    //开启了r2d2连接池、tokio支持
 *
 * 要打开连接，你需要创建一个客户端，然后从它那里获取一个连接。
 */

#[cfg(test)]
mod redis_test {
    use redis::Commands;

    #[test]
    fn string_test() {
        // 1、创建一个客户端
        let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
        // 2、从客户端获取一个连接
        let mut conn = client.get_connection().unwrap();
        let string_result: i32 = conn.set("rust", 123_i32).unwrap();
        println!("string_result: {}", string_result); // 返回的是value值

        let value: i32 = conn.get("rust").unwrap();
        println!("value: {}", value);
    }

    #[test]
    fn hash_test() {
        let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
        let mut conn = client.get_connection().unwrap();
        let hash_result: usize = conn.hset("hash", "name", "rust").unwrap();

        println!("hash_result: {}", hash_result); // 1 表示成功

        let name: String = conn.hget("hash", "name").unwrap();
        println!("{}", name);
    }

    #[test]
    fn list_test() {
        let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
        let mut conn = client.get_connection().unwrap();

        let push_result: usize = conn.lpush("list", 1).unwrap();
        println!("push_result: {}", push_result); // 1 表示成功

        let list: Vec<i32> = conn.lrange("list", 0, -1).unwrap();
        for item in list {
            println!("{}", item);
        }
    }

    #[test]
    fn set_test() {
        let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
        let mut conn = client.get_connection().unwrap();

        let insert_result: usize = conn.sadd("set", "set_test").unwrap();
        println!("insert_result: {}", insert_result); // 1 表示成功

        let list: Vec<String> = conn.smembers("set").unwrap();
        for item in list {
            println!("{}", item);
        }
    }

    #[test]
    fn delete_test() {
        let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
        let mut conn = client.get_connection().unwrap();

        // del可以删除redis中所有类型的key
        let delete_result: usize = conn.del("rust").unwrap();
        println!("delete_result: {}", delete_result); // 1 表示成功

        // 删除Hash类型里面的key
        let hash_del: usize = conn.hdel("hash", "name").unwrap();
        println!("hash_del: {}", hash_del); // 1 表示成功
    }
}

/***********************************高级操作：Stream**************************************/
#[cfg(test)]
mod advance_test {

    use redis::{
        Commands,
        streams::{StreamId, StreamKey, StreamMaxlen, StreamReadOptions, StreamReadReply},
    };

    // 消息生产者
    #[tokio::test]
    async fn stream_producer_test() {
        let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
        let mut conn = client.get_connection().unwrap();

        let message = "this is a stream message";

        // * 表示自动生成id
        let message_id: String = conn.xadd("stream", "*", &[("value", message)]).unwrap();
        println!("message_id: {}", message_id);
    }

    // 消息消费者
    #[tokio::test]
    async fn stream_consumer_test() {
        let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
        let mut conn = client.get_connection().unwrap();

        // 创建消费者组，如果不存在就会创建，$表示从最新的开始消费，0表示从最早的开始消费
        let xgroup: Result<(), _> = conn.xgroup_create_mkstream("stream", "consumer_group_1", "0");
        match xgroup {
            Ok(_) => println!("消费者创建成功"),
            Err(e) => println!("消费者创建失败: {:?}", e),
        }
        let options = StreamReadOptions::default()
            .block(1000)
            .count(1)
            .group("consumer_group_1", "test_1");
        // > 的含义是：返回当前消费者组中尚未被任何消费者处理的新消息
        let read: StreamReadReply = conn.xread_options(&["stream"], &[">"], &options).unwrap();
        if !read.keys.is_empty() {
            for StreamKey { key, ids } in read.keys {
                println!("========={}=========", key);
                for StreamId { id, map } in ids {
                    map.iter().for_each(|(key, value)| {
                        println!("{}: {:?}", key, value);
                    });
                    let ark: usize = conn.xack(&["stream"], "consumer_group_1", &[id]).unwrap();
                    println!("ark: {}", ark); // 1 表示成功
                }
            }
        } else {
            println!("当前key没有消息");
        }
    }

    // // 获取待处理的消息条数
    // #[tokio::test]
    // async fn pending_test(){
    //     let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
    //     let mut conn = client.get_connection().unwrap();

    //     let pending_info: redis::Value = conn.xpending("stream", "consumer_group_1").unwrap();
    //     println!("pending_info: {:?}", pending_info);

    //     if let redis::Value::BulkString(bytes) = pending_info {
    //         let pending_info = String::from_utf8(bytes).unwrap_or_default();
    //         println!("pending_info: {}", pending_info);
    //     }
    // }

    #[tokio::test]
    async fn delete_group() {
        let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
        let mut conn = client.get_connection().unwrap();

        // 1、删除消费者组
        let delete_group_result: usize = conn
            .xgroup_destroy::<&str, &str, usize>("stream", "consumer_group_1")
            .unwrap();
        println!("delete_group_result: {}", delete_group_result); // 1 表示删除成功

        // 2、清空流
        let trim_result: usize = conn.xtrim("stream", StreamMaxlen::Equals(0)).unwrap();
        println!("trim_result: {}", trim_result); // 流中的数据条数
    }
}
