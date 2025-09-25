/**
 * crossterm = "0.29.0"
 *
 * Crossterm 是一个纯 Rust 编写的终端操作库，它使得编写跨平台的文本界面成为可能（见特性）。
 * 它支持所有 UNIX 和 Windows 7 及以上版本的终端（并非所有终端都经过测试，更多信息请见已测试终端）
 *
 */

#[cfg(test)]
mod crossterm_test {
    use crossterm::ExecutableCommand;
    use crossterm::execute;
    use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};

    #[test]
    fn test_1() {
        execute!(
            std::io::stdout(),
            SetBackgroundColor(Color::Red),
            Print("hello world")
        )
        .unwrap();
        std::io::stdout()
            .execute(SetForegroundColor(Color::Blue))
            .unwrap()
            .execute(SetBackgroundColor(Color::Red))
            .unwrap()
            .execute(Print("Styled text here."))
            .unwrap()
            .execute(ResetColor)
            .unwrap();
    }
}
