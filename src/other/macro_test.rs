/**
 * 过程宏的测试代码
 */
#[cfg(test)]
mod test {
    use rust_macro::make_greeting;

    /**
     * 函数宏测试
     */
    #[test]
    fn function_macro_test() {
        make_greeting!("Hello");
        show();
    }

    /**
     * 派生宏测试
     */
    use rust_macro::HelloMacro;

    trait HelloMacro {
        fn hello_macro();
    }

    #[derive(HelloMacro)]
    struct A;

    #[test]
    fn derive_macro_test() {
        A::hello_macro();
    }

    /**
     * 属性宏测试
     */
    use rust_macro::hello_macro_attr;

    #[hello_macro_attr]
    fn test_attr() {
        println!("hello attr");
    }

    #[test]
    fn attr_macro_test() {
        test_attr();
    }
}
