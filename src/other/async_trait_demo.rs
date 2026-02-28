
use async_trait::async_trait;

/**
 * 原生 async fn trait 依然有一个常见限制：不能直接当 trait object 用。比如你一旦写类似
 * async fn f(c: &dyn Condition) -> bool {
 *     c.is_met().await
 * }
 * 这样的代码，Rust 会报错：`the trait `Condition` cannot be made into an object`。
 * 这是因为 async fn trait 底层是用 Future 实现的，而 Future 是不能当 trait object 用的。
 */

#[async_trait]
#[allow(dead_code)]
trait Condition {
    async fn is_met(&self) -> bool;
}

#[allow(dead_code)]
struct A;

#[async_trait]
impl Condition for A {
    async fn is_met(&self) -> bool {
        true
    }
}

#[allow(dead_code)]
struct B;

#[async_trait]
impl Condition for B {
    async fn is_met(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod test{

    use crate::other::async_trait_demo::{Condition, A, B};

    #[tokio::test]
    async fn show() {
        let a = A;
        println!("{}", get_result(&a).await);

        let b = B;
        println!("{}", get_result(&b).await);
    }

    async fn get_result(condition: &dyn Condition) -> bool {
        condition.is_met().await
    }
}