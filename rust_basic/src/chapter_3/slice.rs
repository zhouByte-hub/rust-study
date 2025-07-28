// 切片并不是 Rust 独有的概念，在 Go 语言中就非常流行，它允许你引用集合中部分连续的元素序列，而不是引用整个集合。
/**
 * 创建切片的语法，使用方括号包括的一个序列：[开始索引..终止索引]，其中开始索引是切片中第一个元素的索引位置，而终止索引是最后一个元素后面的索引位置。
 * 换句话说，这是一个 右半开区间（或称为左闭右开区间）——指的是在区间的左端点是包含在内的，而右端点是不包含在内的
 */
pub fn test_1() {
    let content = String::from("content");
    let result = &content[0..3];
    println!("{}", result);

    let all = &content[..];
    println!("{}", all); // 全部截取
}

// 切片的索引必须落在字符之间的边界位置
pub fn test_2() {
    let content = String::from("中国人");

    // 每个汉字占用三个字节，因此没有落在边界处，也就是连 中 字都取不完整，此时程序会直接崩溃退出
    let result = &content[0..2];
    println!("{}", result);
}

pub fn test_3() {
    // 集合的切片
    let list = [1, 2, 3, 4, 5, 6];
    let result = &list[0..3];
    println!("{:?}", result);

    let a = (1, 2, 3, 4, 5);
    // let b = &a[0..3]; // tuple没有切片
    println!("{:?}", a);
}
