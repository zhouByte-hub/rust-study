/** 智能指针的名称来源，主要就在于它实现了 Deref 和 Drop 特征，这两个特征可以智能地帮助我们节省使用上的负担：
 *  Deref 可以让智能指针像引用那样工作，这样你就可以写出同时支持智能指针和引用的代码
 */

// 常规引用是一个指针类型，包含了目标数据存储的内存地址。对常规引用使用 * 操作符，就可以通过解引用的方式获取到内存地址对应的数据值
fn test_1() {
    let x = 5;
    let y = &x;

    println!("{}", *y); // 通过解引用才能真实的值
}

use std::ops::Deref;
// 对于引用或者智能指针来说，本质上都是实现了Deref特征，因此都可以通过 * 操作符来获取到它们所指向的数据值
struct MyBox<T>(T); // 定义元组结构体

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0 // 取出元组结构体的一个值
    }
}

// 当我们对智能指针 Box 进行解引用时，实际上 Rust 为我们调用了以下方法: *(y.deref())
fn test_2() {
    let x = MyBox(5);
    println!("{}", *x); // 当结构体没有实现Deref特征时，不能直接使用 * 操作符
}
