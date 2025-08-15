
/**
 * anyhow = "1.0.99"
 * 
 * 该库提供 anyhow::Error ，一种基于特征对象的错误类型，用于在 Rust 应用程序中轻松进行惯用的错误处理。
 */

#[cfg(test)]
mod anyhow_test{
    use anyhow::{Context, Ok, Result};

    /**
     * 在函数中，使用 ? 可以轻松传播任何实现了 std::error::Error 特质的错误。但是换成其他错误就需要进行指定（鸡肋）
     */
    #[test]
    fn test() -> Result<()> {
        // 唯一有点用的就是上下文，可以帮助调试代码
        std::fs::read_to_string("abc").with_context(|| {
            format!("出现了错误")
        })?;
        Ok(())
    }
}