pub mod form;
pub mod json;
/**
 * Actix Web 提供了一种类型安全的请求信息访问机制，称为提取器（即 impl FromRequest ）。有许多内置的提取器实现（参见实现者）。
 * 1、接受路径参数：path: web::Path<(u32, String)>
 * 2、Query<T> 类型提供了提取请求查询参数的功能
 * 3、Json<T> 允许将请求体反序列化为一个结构体。要从请求体中提取类型信息， T 类型必须实现 serde::Deserialize
 * 4、web::Form的表单正文可以提取到一个结构体中，类似于 Json<T> 。这种类型必须实现 serde::Deserialize 。
 * 5、HttpRequest - HttpRequest 本身就是一个提取器，如果你需要访问请求的其他部分。
 * 6、Bytes - 你可以将请求的有效负载转换为 Bytes
 * 7、Payload - 低级有效负载提取器，主要用于构建其他提取器。
 */
pub mod path;
pub mod query;
