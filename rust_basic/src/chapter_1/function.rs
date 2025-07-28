/** 函数要点
 *      1、函数名和变量名使用蛇形命名法(snake case)，例如 fn add_two() {}
 *      2、函数的位置可以随便放，Rust 不关心我们在哪里定义了函数，只要有定义即可
 *      3、Rust 是静态类型语言，每个函数参数都需要标注类型
 *      4、函数的返回值就是函数体最后一条表达式的返回值，当然我们也可以使用 return 提前返回
 *      5、一个函数如果没有返回值的情况下，默认是会返回一个单元类型作为返回值
 */

 pub fn test(){
    let value = test_1();
    let a = test_2(value); // a == ()
    println!("a = {:?}", a);
 }


 // 返回指定类型的返回值
 pub fn test_1() -> String {
    String::from("abc")
 }

 // 没有返回值的情况下会默认返回一个()单元类型
 pub fn test_2(value: String){
    println!("{value}");
 }
