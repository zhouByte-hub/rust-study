/**
 * ctor = "0.5.0"
 * 
 * Rust 的哲学是，所有操作都在 main 之前或之后发生，而这个库明确地颠覆了这一点。在 ctor 和 dtor 函数中运行的代码应小心限制自己只使用 libc 函数和不需要依赖 Rust 的 stdlib 服务的代码。
 */

#[cfg(test)]
mod ctor_test {

    use ctor::{ctor, dtor};

    #[ctor]
    fn launch() {
        println!("launch function running...");
    }

    #[dtor]
    fn destory(){
        println!("destory function running...");
    }

    #[test]
    fn ctor() {
        println!("test function running...");
    }
}