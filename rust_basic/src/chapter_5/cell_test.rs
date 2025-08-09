use std::cell::{Cell, RefCell};

/**
 *  Rust 提供了 Cell 和 RefCell 用于内部可变性，简而言之，可以在拥有不可变引用的同时修改目标数据，对于正常的代码实现来说，这个是不可能做到的（要么一个可变借用，要么多个不可变借用）。
 *  内部可变性的实现是因为 Rust 使用了 unsafe 来做到这一点，但是对于使用者来说，这些都是透明的，因为这些不安全代码都被封装到了安全的 API 中
 */

struct A;

fn test_1() {
    // 对于实现了Copy的类型，Cell和RefCell没有区别，因为Copy类型在赋值时，会进行深拷贝，所以不会出现多个可变引用的情况
    let a = Cell::new("String");
    println!("{}", a.get());
    a.set("abcdef");
    println!("{}", a.get());

    // 如果值没有实现Copy，则会报错
    // let a = Cell::new(A);
    // a.set(A);
    // println!("{}", a.get());

    // RefCell可以应用于任何类型，包括没有实现Copy的类型
    let b = RefCell::new(String::from("String"));
    let _ = b.borrow(); // 获取不可变引用
    let mut mut_b = b.borrow_mut(); // 获取可变引用
    mut_b.push_str("123"); // 修改值

    let x = RefCell::new(A);
    let mut y = x.borrow_mut();
    *y = A;
}
