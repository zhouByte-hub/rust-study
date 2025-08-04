/**
 * Drop 允许你指定智能指针超出作用域后自动执行的代码，例如做一些数据清除等收尾工作。
 * Drop的执行顺序：
 *      1、变量级别，按照逆序的方式，_x 在 _foo 之前创建，因此 _x 在 _foo 之后被 drop
 *      2、结构体内部，按照顺序的方式，结构体 _x 中的字段按照定义中的顺序依次 drop
 * 我们无法为一个类型同时实现 Copy 和 Drop 特征。因为实现了 Copy 特征的类型会被编译器隐式的复制，因此非常难以预测析构函数执行的时间和频率。因此这些实现了 Copy 的类型无法拥有析构函数。
 */

 #[derive(Debug)]
struct A;

#[derive(Debug)]
struct B;

impl Drop for A {

    fn drop(&mut self) {
        println!("A is dropped");
    }
}

impl Drop for B {

    fn drop(&mut self) {
        println!("B is dropped");
    }
}

#[derive(Debug)]
struct Foo {
    a: A,
    b: B
}

impl Drop for Foo {

    fn drop(&mut self) {
        println!("Foo is dropped");
    }
}


#[cfg(test)]
mod drop_test{

    use crate::chapter_5::drop_point::{Foo, A, B};

    #[test]
    fn test(){
        let foo = Foo {
            a: A,
            b: B
        };
        println!("{:?}", foo);
    }
}