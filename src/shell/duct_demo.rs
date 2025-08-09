use duct::cmd;
use std::fs::File;
use std::{
    io::{BufRead, BufReader},
    path::Path,
};

/**
 * Duct 是用于运行子进程的库。Duct 可以轻松构建管道和重定向 IO，就像 shell 一样。
 * 同时，Duct 可以帮助您编写正确的可移植代码：空格从来都不重要，默认情况下会报告来自子进程的错误，以及各种陷阱、错误和平台不一致 以正确的方式™为您处理。
 */
pub fn duct_test() {
    // 不捕捉任何输出
    cmd!("ls", "-a").run().unwrap();

    // 捕捉命令的标准输出
    let result = cmd!("ls").read().unwrap();
    println!("{}", result);
}

pub fn duct_test_2() {
    // 捕获管道的标准输出，将第一个命令的输出作为第二个命令的数据
    let _result = cmd!("ls", "-a")
        .pipe(cmd!("sort")) // 可以换成其他命令，如 grep, sed 等
        .stdout_file(File::create_new(Path::new("file.txt")).unwrap()) // 将最终输出重定向到文件
        .run()
        .unwrap(); // 执行整个管道
}

pub fn duct_test_3() {
    // 将标准错误合并到标准输出
    let output = cmd!("bash", "-c", "echo out && echo err 1>&2")
        .stderr_to_stdout()
        .reader()
        .unwrap();
    let buffer = BufReader::new(output);
    let mut lines = buffer.lines();
    println!("out = {:?}", lines.next());
    println!("error = {:?}", lines.next());
}
