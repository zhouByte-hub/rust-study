use thiserror::Error;

/**
 * thiserror = "2.0.14"
 *
 * 为标准库的 std::error::Error 特性提供了一个便捷的 derive 宏。
 * 每个包含 #[from] 属性的变体，会生成一个 From 实现；如果没有指定from则可以自定义实现
 *
 * 如果你关心设计自己专用的错误类型，以便在失败时调用者能接收到你选择的精确信息，那么请使用 thiserror。
 * 这通常适用于库代码。如果你不关心函数返回什么错误类型，你只想让它变得容易使用，那么请使用 anyhow。这在应用程序代码中较为常见。
 *
 * #[error]支持的参数：
 *      1、#[error("{var}")] ⟶ write!("{}", self.var)
 *      2、#[error("{0}")] ⟶ write!("{}", self.0)
 *      3、#[error("{var:?}")] ⟶ write!("{:?}", self.var)
 *      4、#[error("{0:?}")] ⟶ write!("{:?}", self.0)
 */

#[derive(Error, Debug)]
#[allow(dead_code)]
enum StudyError {
    // #[error("data store disconnected")]
    // Disconnect(#[from] std::io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),

    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },

    #[error("unknown data store error")]
    Unknown,

    #[error("ReconcileError: {0}")]
    ReconcileError(String),
}

/*
    要么使用#[from]要么使用impl
*/
impl From<std::io::Error> for StudyError {
    fn from(_value: std::io::Error) -> Self {
        StudyError::ReconcileError("IO异常".into())
    }
}
