/**
 * derive_builder = "0.20.2"
 *
 * 一个 Rust 宏，用于自动为任意结构体实现 Builder 模式。一个简单的 #[derive(Builder)] 将为您结构体 Foo 生成 FooBuilder ，包含所有设置方法和一个构建方法。
 */

#[cfg(test)]
mod builder_test {
    use derive_builder::Builder;

    #[derive(Debug, Builder)]
    #[allow(dead_code)]
    struct User {
        username: String,
        age: u8,
        address: String,
    }

    #[test]
    fn test_1() {
        let user = UserBuilder::default()
            .username("zhangsan".to_string())
            .age(12)
            .address("beijing".to_string())
            .build()
            .unwrap();
        println!("{:?}", user);
    }
}
