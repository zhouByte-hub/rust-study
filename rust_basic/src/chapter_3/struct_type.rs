use std::fmt::Debug;

/**
 * 结构体跟之前讲过的元组有些相像：都是由多种类型组合而成。但是与元组不同的是，结构体可以为内部的每个字段起一个富有含义的名称。因此结构体更加灵活更加强大，你无需依赖这些字段的顺序来访问和解析它们。
 */

#[derive(Debug)]
#[allow(dead_code)]
struct User {
    username: String,
    password: String,
}

pub fn test() {
    // 初始化实例时，每个字段都需要进行初始化；初始化时的字段顺序不需要和结构体定义时的顺序一致
    let user = User {
        username: String::from("张三"),
        password: String::from("123123"),
    };

    // .. 语法表明凡是我们没有显式声明的字段，全部从 user1 中自动获取。需要注意的是 ..user1 必须在结构体的尾部使用。
    let user2 = User {
        username: String::from("李四"),
        ..user
    };
    // 把结构体中具有所有权的字段转移出去后，将无法再访问该字段，但是可以正常访问其它的字段。
    // println!("{}-{}", user.username, user.password);     // 会报错，因为所有权进行了转移
    println!("{:?}", user2)
}

// 元组结构体
#[derive(Debug)]
#[allow(dead_code)]
pub struct ColorStruct(u8, u8, u8);

pub fn test_2() {
    let red = ColorStruct(255, 0, 0);
    println!("{:?}", red)
}

// 单元结构体：定义一个类型，但是不关心该类型的内容，只关心它的行为时，就可以使用单元结构体
pub struct UnitStruct;

impl UnitStruct {
    pub fn test() {
        println!("单元结构体")
    }
}

/** 结构体所有权
 * 有一处细节：我们使用了自身拥有所有权的 String 类型而不是基于引用的 &str 字符串切片类型。这是一个有意而为之的选择：因为我们想要这个结构体拥有它所有的数据，而不是从其它地方借用数据。
 * 生命周期能确保结构体的作用范围要比它所借用的数据的作用范围要小。
 */
#[allow(dead_code)]
struct Person<'a> {
    username: &'a str, // 表示当前结构体中的 username 是借用了外部的值，Rust 中的所有权会保证结构体的中 username 的作用域小于原始值
}

impl Debug for Person<'_> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
