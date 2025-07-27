/**
 * iN, uN 和 fN 占用 N 位，
 * isize 和 usize 占用一个指针大小的空间，
 * char 占用 32 位空间，
 * bool 占用 8 位空间。
 */

// Rust中声明变量时也可以不声明类型，Rust 会根据变量的使用来确定其类型。 

pub fn sign_integer() {
    // 有符号整数(只有有符号整数才会存在负数)
    let a:i8 = 12;
    let b: i16 = -24;
    let c: i32 =  48;
    let d: i64 = -96;
    let e: i128 = 192;
    let f: isize = -384;


    println!("{} {} {} {} {} {}", a, b, c, d, e, f)
}

pub fn unsigned_integer() {
    // 无符号整数
    let a: u8 = 12;
    let b: u16 = 24;
    let c: u32 = 48;
    let d: u64 = 96;
    let e: u128 = 192;
    let f: usize = 384;

    println!("{} {} {} {} {} {}", a, b, c, d, e, f)
}


pub fn float_type() {
    // 浮点数，值要么是浮点数要么就加上标识符 f32 或者 f64
    let a: f32 = 12_f32;
    let b: f64 = 24.0;

    println!("{} {}", a, b)
}

pub fn unicode_type() {
    // 字符类型，可以是任何Unicode字符
    let a: char= 'a';
    println!("{}", a)
}

pub fn boolean_type() {
    let a: bool = true;
    println!("{}", a)
}

// 数字中的所有下划线均可忽略，它们只是为了方便辨识。因此，1_000 可以写为 1000（或 10_00），而 123_i64 可以写为 123i64。


