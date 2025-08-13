/**
 * serde = {version="1.0", features = ["derive"]}
 *
 * Serde 是一个用于高效、通用地对 Rust 数据结构进行序列化和去序列化的框架。
 * Serde 生态系统由知道如何序列化和反序列化自身的数据结构以及知道如何序列化和反序列化其他事物的数据格式组成。
 * Serde 提供了这两个组相互交互的层，允许使用任何支持的数据格式对任何支持的数据结构进行序列化和反序列化
 *
 * Serde只是定义了序列化和反序列话的trait，具体的序列化方式还得引入Serde的实现依赖
 */
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    username: String,
    age: i32,
}

#[cfg(test)]
mod test {

    #[test]
    fn test_1() {}
}
