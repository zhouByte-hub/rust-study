pub mod anyhow_demo;
pub mod thiserror_demo;

/*
   thisError可以自定义类型，但是没有上下文记录
   anyhow有上下文记录，但是只能抛出继承std::io::Error类型的错误

   所以一般可以两者结合使用：thisError的自定义类型 + anyhow的上下文
*/
