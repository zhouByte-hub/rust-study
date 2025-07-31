// 枚举类型是一个类型，它会包含所有可能的枚举成员，而枚举值是该类型中的具体某个成员的实例。

#[derive(Debug)]
#[allow(dead_code)]
enum Person {
    // 修正拼写
    Developer,
    Manager,
}

pub fn test() {
    let dev = Person::Developer; // 修正拼写
    println!("{:?}", dev);
}

// 任何类型的数据都可以放入枚举成员中
#[derive(Debug)]
#[allow(dead_code)]
enum EnumTest {
    A, // 无关联数据枚举
    B {
        // 匿名结构体枚举
        x: i32,
        y: i32,
    },
    C(String),     // 包含字符串枚举
    D(u8, u8, u8), // 元组结构体枚举
}

// 枚举的实现更简洁，代码内聚性更强，不像结构体的实现，分散在各个地方。
pub fn test_2() {
    let a = EnumTest::A;
    let b = EnumTest::B { x: 32, y: 64 };
    let c = EnumTest::C(String::from("abc"));
    let d = EnumTest::D(255, 255, 255);

    println!("{:?}-{:?}-{:?}-{:?}", a, b, c, d);
}
