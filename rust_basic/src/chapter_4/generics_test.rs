// 用同一功能的函数处理不同类型的数据，例如两个数的加法，无论是整数还是浮点数，甚至是自定义类型，都能进行支持。在不支持泛型的编程语言中，通常需要为每一种类型编写一个函数
// 泛型就是一种多态。泛型主要目的是为程序员提供编程的便利，减少代码的臃肿，同时可以极大地丰富语言本身的表达能力，为程序员提供了一个合适的炮管。
// 泛型是在编译期完成处理的：编译器会为每一个泛型参数对应的具体类型生成一份代码，这种方式是静态分发(static dispatch)，因为是在编译期完成的，对于运行期性能完全没有任何影响。

use std::ops::Add;

// 泛型方法：泛型加载方法名后面
/** 当对泛型参数进行运算时，需要考虑以下情况：
 *  1、加法：std::ops::Add
 *  2、减法：std::ops::Sub<Output = T
 *  3、乘法：std::ops::Mul<Output = T>
 *  4、除法：std::ops::Div<Output = T>
 *  5、比较：std::cmp::PartialOrd
 *
 */
pub fn add<T: Add<Output = T>>(value1: T, value2: T) -> T {
    value1 + value2
}

// 结构体泛型
pub struct Test<T>
where
    T: std::clone::Clone,
{
    username: T,
}

impl<T: std::clone::Clone> Test<T> {
    pub fn get_username(&self) -> T {
        self.username.clone()
    }
}

// 枚举结构体
#[derive(Debug)]
pub enum TestEnum<T>
where
    T: std::fmt::Debug,
{
    A(T),
}

impl<T> TestEnum<T>
where
    T: std::fmt::Debug,
{
    pub fn show(&self) -> String {
        format!("{:?}", self)
    }
}
