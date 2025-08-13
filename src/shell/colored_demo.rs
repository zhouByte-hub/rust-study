/**
 * colored = "3.0.0"
 *
 * 在终端中添加颜色的最简单方法
 * 支持的颜色：
 *      black  黑色
 *      red    红色
 *      green  绿色
 *      yellow 黄色
 *      blue   蓝色
 *      magenta 洋红色
 *      cyan   青色
 *      white  白色
 *
 * 亮色：在颜色前加上 bright_
 * 背景色：在颜色前加上 on_
 * 亮背景色：在颜色前加上 on_bright_
 *
 * 支持的字体：
 *      bold  粗体
 *      underline  下划线
 *      italic  斜体
 *      dimmed  暗淡
 *      reversed  反转
 *      blink  闪烁
 *      hidden  隐藏
 *      strikethrough  删除线
 */

#[cfg(test)]
mod color_test {

    use colored::Colorize;

    #[test]
    fn test() {
        println!("{}", "this is blue".blue()); // 蓝色
        println!("{}", "this is red".red()); // 红色
        println!("{}", "this is green".green()); // 绿色
        println!("{}", "this is yellow".yellow()); // 黄色
        println!("{}", "this is magenta".magenta()); // 紫色（品红）
        println!(
            "{}",
            "this is bright colors are welcome as well"
                .on_bright_blue()
                .bright_red()
        ); // 亮蓝色背景，红色文字

        println!("{}", String::from("this also works!").green().bold()); // 绿色加粗
    }
}
