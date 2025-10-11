/**
 * enum_dispatch = "0.3.13"
 * 
 * 使用枚举来动态调用不同的实现方法，几乎可以替代动态调度方法调用，速度提高 10 倍。
 */

#[cfg(test)]
mod base_test{

    // 普通的动态调用
    trait CommonTrait{
        fn common_method(&self);
    }

    struct BaseCommon1;

    impl CommonTrait for BaseCommon1 {
        fn common_method(&self) {
            println!("BaseCommon1 common_method");  
        }
    }

    #[test]
    fn test(){
        /*
            1. Box是一个智能指针，表示将对象分配置堆上，因为在编译的时候不知道具体的类型从而不知道具体的大小。
            2. 使用dyn表示动态引用，因为在编译的时候不知道具体的类型，所以只能使用动态引用。
            3. Box + dyn 形成了一种类似动态多态的机制。
         */
        let value: Box<dyn CommonTrait> = Box::new(BaseCommon1{});
        show(&value);
    }

    fn show(obj: &Box<dyn CommonTrait>) {
        obj.common_method();
    }
}


/**
 * 枚举调度演示
 */

#[cfg(test)]
mod menu_dispatch_test{

    use enum_dispatch::enum_dispatch;

    #[derive(Debug)]
    #[enum_dispatch]
    enum CommonMenu{
        BaseCommon1,
    }

    #[enum_dispatch(CommonMenu)]
    trait CommonTrait{
        fn common_method(&self);
    }

    struct BaseCommon1;

    impl CommonTrait for BaseCommon1 {
        fn common_method(&self) {
            println!("BaseCommon1 common_method");  
        }
    }

    impl BaseCommon1 {
        fn new() -> Self {
            Self {}
        }
    }

    #[test]
    fn test(){
        // 利用实现类获取到枚举对象，通过枚举对象调用方法
        let menu: CommonMenu = BaseCommon1::new().into();
        menu.common_method();
    }
}