/**
 * iN, uN 和 fN 占用 N 位，
 * isize 和 usize 占用一个指针大小的空间，
 * char 占用 32 位空间，
 * bool 占用 8 位空间。
 *
 * 有符号整数范围： -(2n - 1) ~ 2n - 1 - 1
 * 无符号整数范围： 0 ~ 2n - 1
 */

pub fn sign_integer() {
    // 有符号整数(只有有符号整数才会存在负数)
    let a: i8 = 12;
    let b: i16 = -24;
    let c: i32 = 48;
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

/** 整形的溢出
 * 使用 wrapping_* 方法在所有模式下都按照补码循环溢出规则处理，例如 wrapping_add
 * 如果使用 checked_* 方法时发生溢出，则返回 None 值
 * 使用 overflowing_* 方法返回该值和一个指示是否存在溢出的布尔值
 * 使用 saturating_* 方法，可以限定计算后的结果不超过目标类型的最大值或低于最小值
 */
pub fn integer_outflow() {
    // 这里会发生溢出，但是不报错
    let mut i: i8 = 127;
    i += 1;
    println!("{}", i);

    // 溢出提示
    let a: i8 = 127;
    let b = a.wrapping_add(12);
    let c = b.checked_add(12);
    let d = c.unwrap_or_else(|| 0).overflowing_add(32);
    let e = if d.1 { d.0.saturating_add(12) } else { d.0 };
    println!("{}", e);
}
