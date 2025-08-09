/** 闭包是一种匿名函数，它可以赋值给变量也可以作为参数传递给其它函数，不同于函数的是，它允许捕获调用者作用域中的值
 *  闭包的三种fn特征：
 *      1、FnOnce：该类型的闭包会拿走被捕获变量的所有权，并且该闭包只能被运行一次。
 *      2、FnMut：它以可变借用的方式捕获环境中的值。
 *      3、Fn：它以不可变借用的方式捕获环境中的值。
 *
 *  闭包三种fn特征的关系：
 *      1、所有的闭包都实现了FnOnce特征，也就是说任何闭包都至少可以被调用一次。
 *      2、没有移出所捕获变量的所有权的闭包自动实现了FnMut特征。
 *      3、不需要对捕获变量进行改变的闭包自动实现了Fn特征。
 */

fn test_1() {
    let a = |x: i32, y: i32| x + y;
    let _result = a(1, 2);
}

// 方法中使用闭包
fn test_2<T>(add: T) -> i32
where
    T: Fn(i32, i32) -> i32,
{
    add(1, 2)
}

/** 闭包作为函数的返回值
 *  对于无法在编译期间确定的类型，都需要使用dyn来表示动态类型，但是使用impl作为参数返回值时就不需要。
 *  因为返回值是Fn(i32, i32) -> i32的一个实现，这个返回值在编译期间就可以确定下来，所以不需要写dyn。
 *
 *  一句话总结：
 *  在函数参数中使用 impl Fn(i32, i32) -> i32 是 “泛型 + Trait Bound” 的语法糖，
 *  它表示“这个参数可以是任何实现了 Fn(i32, i32) -> i32 trait 的类型”，
 *  编译器会在调用时进行 单态化（monomorphization），生成具体类型的版本，属于 静态分发，因此不需要 dyn。
 */
fn test_4() -> impl Fn(i32, i32) -> i32 {
    let m = 5;
    move |a, b| a + m + b
}

// 也不需要写dyn
fn a(_value: impl Fn(i32, i32) -> i32) {}

// 结构体中使用闭包
struct User<T>
where
    T: Fn(i32) -> i32,
{
    query: T,
}

/** 闭包获取上下文环境中的值
 * 当闭包从环境中捕获一个值时，会分配内存去存储这些值。对于有些场景来说，这种额外的内存分配会成为一种负担。
 */
fn test_3() {
    let a = String::from("abc");
    let _add = |x: &str| format!("{}-{}", a, x); // 捕获上下文件环境中的a

    // 强制闭包取得被捕获变量的所有权
    let _sub = move |x: i32| format!("{}-{}", a, x);

    // println!("{}", a); // 这里会报错，因为a的所有权已经被转移了
}

/** 迭代器
 *  1、iter()：得到一个借用迭代器
 *  2、into_iter()：会获取元素的所有权
 *  3、iter_mut()：得到一个可变借用迭代器
 */
fn for_test() {
    let list = [1, 2, 3, 4];
    // 1、普通遍历
    for item in list {
        println!("{}", item)
    }

    // 2、迭代器遍历：迭代器是惰性的，意味着如果你不使用它，那么它将不会发生任何事
    for item in list.iter() {
        println!("{}", item)
    }

    // 3
    let mut iter = list.iter();
    println!("{:?}", iter.next()); // 使用next方法一个一个读取元素

    // 4
    // for item in list.iter_mut(){} // 要求list是mut

    // 5
    for item in list.into_iter() {
        println!("{}", item)
    }

    // 1和5一样，都会转移所有权
}
