// Rust中的方法的表现需要于impl进行结合

#[derive(Debug)]
pub struct Person {
    username: String,
    age: u8,
}

/**
 * self/Self 表示 Person 的所有权转移到该方法中，这种形式用的较少
 * &self 表示该方法对 Rectangle 的不可变借用
 * &mut self 表示可变借用
 */
impl Person {
    // 对象方法，可以通过对象.方法名称的方式调用
    pub fn get_username(&self) -> &String {
        &self.username
    }

    pub fn get_age(&self) -> u8 {
        self.age
    }

    // 内联方法，需要通过结构体::内联方法的方式调用
    pub fn new(username: String, age: u8) -> Self {
        Person { username, age }
    }
}

// 可以根据职责将方法拆分到多个impl中
impl Person {
    pub fn to_school() {}
}

#[derive(Debug)]
enum Color {
    RED,
}

// 为枚举对象实现方法，每个枚举对象都会有这个方法
impl Color {
    pub fn show(&self) -> String {
        format!("{}-{:?}", "type", self)
    }
}

// 与常量对应的常量方法
// const fn 允许我们在编译期对函数进行求值，从而实现更高效、更灵活的代码设计。
const RESULT: i32 = get_result(10, 20);
const fn get_result(value1: i32, vlaue2: i32) -> i32 {
    value1 + vlaue2
}
println!("{}", RESULT);

#[cfg(test)]
mod test {
    use crate::chapter_4::function::Color;
    use crate::chapter_4::function::Person;

    #[test]
    pub fn test_1() {
        Person::new(String::from("张三"), 12);

        println!("{}", Color::RED.show());
    }
}
