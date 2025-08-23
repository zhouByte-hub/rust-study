/**
 * redis = {version = "0.32.5", features = ["r2d2", "tokio-comp"]}    //开启了r2d2连接池、tokio支持
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
        conn.set::<&str, i32, ()>("rust", 123_i32).unwrap();
        let value: i32 = conn.get("rust").unwrap();
        println!("value: {}", value);
    }

    #[test]
    fn hash_test() {
        let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
        let mut conn = client.get_connection().unwrap();
        conn.hset::<&str, &str, &str, ()>("hash", "name", "rust")
            .unwrap();

        let name: String = conn.hget("hash", "name").unwrap();
        println!("{}", name);
    }

    #[test]
    fn list_test() {
        let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
        let mut conn = client.get_connection().unwrap();

        conn.lpush::<&str, i32, ()>("list", 1).unwrap();

        let list: Vec<i32> = conn.lrange("list", 0, -1).unwrap();
        for item in list {
            println!("{}", item);
        }
    }

    #[test]
    fn set_test() {
        let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
        let mut conn = client.get_connection().unwrap();

        conn.sadd::<&str, &str, ()>("set", "set_test").unwrap();

        let list: Vec<String> = conn.smembers("set").unwrap();
        for item in list {
            println!("{}", item);
        }
    }
}
