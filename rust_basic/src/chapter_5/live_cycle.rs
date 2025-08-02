/** 生命周期，简而言之就是引用的有效作用域
 *  产生生命周期的原因：在存在多个引用时，编译器有时会无法自动推导生命周期，此时就需要我们手动去标注，通过为参数标注合适的生命周期来帮助编译器进行借用检查的分析。
 *      1、在大多数时候，我们无需手动的声明生命周期，因为编译器可以自动进行推导。
 *      2、生命周期的主要作用是避免悬垂引用，它会导致程序引用了本不该引用的数据。
 *      3、生命周期标注并不会改变任何引用的实际作用域。
 *      4、 在通过函数签名指定生命周期参数时，我们并没有改变传入引用或者返回引用的真实生命周期，而是告诉编译器当不满足此约束条件时，就拒绝编译通过。
 */

// pub fn test_1(value1: &str, value2: &str) -> &str{
//     value1
// }

// 上述代码会报错，需要使用下列这种方式，让value1和value2两个引用的生命周期至少能和test_1这个方法的生命周期一样长
pub fn live_cycle_test_1<'a>(value1: &'a str, _value2: &'a str) -> &'a str{
    value1
}

// 结构体生命周期
pub struct User<'a>{
    username: &'a str   // 让username这个引用的生命周期至少和User这个结构体对象的生命周期一样长
}


/* 生命周期的三条消除规则
    1、每一个引用参数都会获得独自的生命周期。
    2、若只有一个输入引用参数，那么该引用参数的生命周期就会默认的赋给输出引用参数。
    3、若存在多个引用参数，且其中一个是&self或者&mut self，则self的生命周期会赋值给所有输入引用参数。
    4、若输出的引用参数与输入引用参数有关，则需要声明生命周期
*/
pub fn live_cycle_test_2(_value1: &str, _value2: &str){ // 符合条件1

}

pub fn live_cycle_test_3(_value1: &str) -> &str{  //符合条件2
    _value1
}

struct A;

impl A {
    
    pub fn live_cycle_test_4(&self, _value1: &str, _value2: &str){  // 符合条件3
        
    }

    pub fn live_cycle_test_5<'a>(&self, value1: &'a str, _value2: &'a str) -> &'a str{  // 符合条件4
        value1
    }
}


// 静态生命周期，存活时间与程序一样久：&'static和'static
pub fn live_cycle_test_6(){
    let _content: &'static str = "abcdef";
}



#[cfg(test)]
mod test_live_cycle{
    use crate::chapter_5::live_cycle::{live_cycle_test_1, User};


    #[test]
    pub fn cycle_test(){
        live_cycle_test_1("a", "b");

        let name = "张三";
        User{
            username: name
        };
    }

}