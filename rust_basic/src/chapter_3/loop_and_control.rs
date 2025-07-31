// if语句
pub fn if_test() {
    let i = 10;
    if i == 10 {
        println!("{}", i);
    } else if i < 10 {
        println!("{}", i);
    } else {
        println!("{}", i)
    }
}

/** for循环
 * 使用 for 时我们往往使用集合的引用形式，除非你不想在后面的代码中继续使用该集合
 * 如果不使用引用的话，所有权会被转移（move）到 for 语句块中，后面就无法再使用这个集合了
 */
pub fn for_test() {
    let mut array = [1, 2, 3, 4, 5];
    // 对于实现了 copy 特征的数组（例如 [i32; 10]）而言， for item in arr 并不会把 arr 的所有权转移，而是直接对其进行了拷贝，因此循环之后仍然可以使用 arr 。
    for item in array {
        println!("{}", item);
    }

    /*
     * iter()：不可变借用
     * iter_mut()：可变借用
     * into_iter()：转移所有权借用
     */
    for item in array.iter() {
        // item的类型是&i32
        println!("{}", item);
    }

    for item in array.iter_mut() {
        // item的类型是&mut i32
        *item += 2;
    }

    for item in array.into_iter() {
        // item的类型是i32，会转移所有权，因为是基本数据类型，所有执行的是copy操作
        println!("{}", item);
    }

    // 获取下标和值
    for (index, value) in array.iter().enumerate() {
        println!("index = {}, value = {}", index, value);
    }
}

// while循环
pub fn while_test() {
    let mut i = 10;
    while i >= 0 {
        println!("{}", i);
        i -= 1;
    }
}

/** loop循环
 * break在loop中可以单独使用，也可以带一个返回值，有些类似 return
 * loop 是一个表达式，因此可以返回一个值
 */
pub fn loop_test() {
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count;
        }
    };
    println!("{}", result);
}
