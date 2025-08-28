pub mod database;
pub mod error;
pub mod file;
pub mod logs;
pub mod other;
pub mod serialize;
pub mod shell;
pub mod system;
pub mod web;
pub mod windows;

fn main() {}


/** env！
 * 在编译时读取系统环境变量的值，并将其作为字符串字面量插入到代码中。
 * 如果指定的环境变量在编译时不存在，编译会直接失败。这保证了程序运行时一定能访问到该值。
 */
#[cfg(test)]
mod main_test{

    #[test]
    fn test_env() {
        let env = env!("LANG");
        println!("{}", env);
    }
}