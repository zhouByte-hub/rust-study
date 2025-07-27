
pub fn if_control_flow() {
    let a = 10;
    // 方式一
    if a == 10 {
        println!("a == 10");
    }else if a > 10 {
        println!("a > 10")
    }else {
        println!("a < 10")
    }

    // if语句可以作用于赋值语句,如果在表达式中使用 if，则表达式中必须包含 ;
    let result = if a == 10 {a * 10} else {a - 10};
    println!("{}", result);
}


