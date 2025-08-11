use clap::{Parser, Subcommand};

/**
 * 一个简单易用、高效且功能全面的命令行参数解析器
 * command参数：
 *      version：自动使用 Cargo.toml 中的 version 字段作为程序版本。
 *      about： 自动使用 Cargo.toml 中的 description 字段作为简短说明。
 *      long_about: 长描述（覆盖默认）
 *      author：作者信息
 *      name：自定义程序名
 *      subcommand：标记为子命令
 *
 * arg参数：
 *      short：支持短选项 -n
 *      long：支持长选项 -name
 *      value_name： 在帮助信息中显示值的占位符 value_name="FILE" 在命令的提示的地方就会有FILE占位符
 *      default_value：设置参数默认值
 *      help：参数说明
 *      required：是否为必填参数
 *      num_args：接受多个值（范围）
 *      action：计数模式，ArgAction::Count
 *      env：从环境变量读取默认值
 *      value_enum：枚举类型参数
 */

#[derive(Parser, Debug)] // Parser用于结构体，自动生成解析命令行参数的代码
#[command(
    name = "clapDemo",
    version,
    about = "A file processing tool",
    long_about = "一个简单易用、高效且功能全面的命令行参数解析器",
    author = "Time Travel"
)]
struct ClapDemo {
    #[arg(short, long, value_name = "NAME")]
    name: Option<String>,

    #[arg(
        short,
        long,
        default_value = "c_type",
        value_name = "type",
        help = "类型",
        required = true
    )]
    c_type: String,

    #[command(subcommand)]
    sub: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    Test,
    Run,
}

pub fn main() {
    let value = ClapDemo::parse();
    println!("{:?}", value);
}
