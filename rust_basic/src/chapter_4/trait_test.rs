// Rust中的Trait就相当于是Java中的接口
// 特征定义了一组可以被共享的行为，只要实现了特征，你就能使用这组行为。
// 当使用特征对象时，Rust 必须使用动态分发。编译器无法知晓所有可能用于特征对象代码的类型，所以它也不知道应该调用哪个类型的哪个方法实现。为此，Rust 在运行时使用特征对象中的指针来知晓需要调用哪个方法。

use std::iter::Sum;

/** trait孤儿原则
 * 如果你想要为类型 A 实现特征 T，那么 A 或者 T 至少有一个是在当前作用域中定义的！
 * 例如我们可以为上面的 Post 类型实现标准库中的 Display 特征，这是因为 Post 类型定义在当前的作用域中。同时，我们也可以在当前包中为 String 类型实现 Summary 特征，因为 Summary 定义在当前作用域中。
 * 但是你无法在当前作用域中，为 String 类型实现 Display 特征，因为它们俩都定义在标准库中，其定义所在的位置都不在当前作用域，跟你半毛钱关系都没有，看看就行了。
 */

pub trait Summary {
    fn summarize(&self) -> String;
}

// 为类实现trait
pub struct Test {
    username: String,
}

impl Summary for Test {
    fn summarize(&self) -> String {
        self.username.clone()
    }
}

// trait作为函数参数：只要是实现了指定trait的对象都可以传入
fn show(item: &impl Summary) -> String {
    item.summarize()
}

// 泛型约束
fn test<T: Summary>() {}
fn test_2<T: Summary + Sum>() {}

// 返回值约束
fn test_3() -> impl Summary {
    Test {
        username: String::from("abc"),
    }
}




/** 关联类型
 * 关联类型是在特征定义的语句块中，声明一个自定义类型，这样就可以在特征的方法签名中使用该类型
 */
pub trait AssisTrait {
    type A;     // 对于实现了AssisTrait的结构体或者枚举来说，都需要显示的声明A的类型

    fn add(value: Self::A, value2: Self::A) -> Self::A;
}

pub struct AssisTest;

/**
 *  也可以使用泛型，但是，当泛型很长的时候且每个地方都要写的时候就会很麻烦，此时就可以使用type来绑定一个类型的别名
 */
impl AssisTrait for AssisTest {
    type A = i32;

    fn add(value: Self::A, value2: Self::A) -> Self::A {
        value + value2
    }
}