/**
 * 宏（Macros）是一种强大的元编程工具，可以用来生成代码、减少重复以及实现复杂的编译时逻辑。Rust中的宏主要分为两种类型：
 *      1、声明宏：也称为macro_rules!宏，用于定义简单的代码生成规则。
 *      2、过程宏：用于定义更复杂的代码生成规则，过程宏可以在编译时对代码进行操作。
 *
 * 宏是在编译时展开的，它们生成代码并插入到调用宏的位置。
 *
 * 宏可以接受多种类型的参数，称为“指示符”：
 *      1、block：代码块，用于多个语句组成的代码块。
 *      2、expr：表达式，可以是任何合法的Rust表达式。
 *      3、ident：标识符，用于变量名、函数名、类型名等。
 *      4、item：项，用于函数、结构体、模块等。
 *      5、literal：字面量，常用于常量值（字符串、数字等）。
 *      6、pat：模式，用于模式匹配。
 *      7、path：路径，用于模块路径、类型路径等。
 *      8、stmt：语句，用于单一语句。
 *      9、tt：令牌树，用于表示Rust代码的语法树。
 *      10、type：类型，用于指定类型名称。
 *      11、vis：可见性，用于指定项的可见性（pub、pub(crate)、pub(in mod)等）。
 */
#[cfg(test)]
mod block_test {

    /**
     * 表达式：能表达一个操作的部分，也就是赋值语句的右侧。
     * 语句：能执行一个操作的部分，有左值、右值。
     * 代码块：由多个语句组成
     */
    // 代码块
    macro_rules! example {
        // 执行代码块
        ($b: block) => {
            $b
        };
    }

    #[test]
    fn test_1() {
        /* 展开为
           {
               let a = 1;
               let b = 2;
               println!("{}", a + b);
           }
        */
        example!({
            let a = 1;
            let b = 2;
            println!("{}", a + b);
        })
    }
}

#[cfg(test)]
mod expr_test {

    // 表达式
    macro_rules! example {
        // 只要是Rust中合法的表达式就可以进行执行
        ($e: expr) => {
            println!("{}", $e);
        };
    }

    #[test]
    fn test() {
        /*  展开为
           println!("{}", 1 + 2);
        */
        example!(1 + 2);
    }
}

#[cfg(test)]
mod ident_test {

    // 标识符，用于变量名、函数名、类型名等。
    macro_rules! example {
        ($i: ident) => {
            let $i = 10;
        };
    }

    #[test]
    fn test() {
        /*  展开为
           let a = 10;
        */
        example!(a);
        println!("{}", a);
    }
}

#[cfg(test)]
mod ty_test {

    // 类型，用于指定类型名称。
    macro_rules! example {
        ($t: ty) => {
            let a: $t = 10;
            println!("{}", a);
        };
    }

    #[test]
    fn test() {
        /* 展开为
           let a: i32 = 10;
           println!("{}", a);
        */
        example!(i32);
    }
}

#[cfg(test)]
mod pat_test {

    // 用于模式匹配。
    macro_rules! example {
        ($e: expr, $p: pat) => {
            match $e {
                $p => println!("{}", $e),
                _ => println!("other"),
            }
        };
    }

    #[test]
    fn test() {
        /*  展开为
           match 10 {
               10 => println!("10"),
               _ => println!("other"),
           }
        */
        example!(10, 10);
    }
}

#[cfg(test)]
mod stmt_test {

    // 语句，用于执行单一语句。
    macro_rules! example {
        ($s: stmt) => {
            $s
        };
    }

    #[test]
    fn test() {
        example!(println!("hello world"));
    }
}

#[cfg(test)]
mod item_test {

    // 项，用于函数、结构体、模块等项。
    macro_rules! example {
        ($i: item) => {
            $i
        };
    }

    #[test]
    fn test() {
        /*  展开为
           fn temp() {
               println!("hello world");
           }
        */
        example!(
            fn temp() {
                println!("hello world");
            }
        );
        temp();
    }
}

#[cfg(test)]
mod meta_test {

    // 元数据项，用于属性：结构体属性、注解等元信息
    macro_rules! example {
        ($i: item, $m: meta) => {
            // #[$m]
            $i
        };
    }

    #[test]
    fn test() {
        /* 展开为
           #[test]
           fn temp() {
               println!("hello world");
           }
        */
        example!(
            fn temp() {
                println!("hello world");
            },
            test
        );
        temp();
    }
}

#[cfg(test)]
mod path_test {

    // 路径，用于路径（例如模块路径）
    macro_rules! example {
        ($p: path) => {
            println!("{}", $p);
        };
    }

    #[test]
    fn test() {
        /* 展开为
           println!("std::fmt::Error");
        */
        example!(std::fmt::Error);
    }
}

#[cfg(test)]
mod literal_test {

    // 字面量，用于常量值（字符串、数字等）。
    macro_rules! example {
        ($l: literal) => {
            let x = $l;
            println!("{}", x);
        };
    }

    #[test]
    fn test() {
        /*  展开为
           let x = "hello world";
           println!("{}", x);
        */
        example!("hello world");
    }
}

#[cfg(test)]
mod vis_test {

    // 可见性描述符
    macro_rules! example {
        ($v: vis) => {
            $v fn temp() {
                println!("hello world");
            }
            temp();
        };
    }

    #[test]
    fn test() {
        example!(pub);
    }
}
