/**
 * flexi_logger = "0.31"
 *
 * 一个灵活且易于使用的日志记录器，可以将日志写入标准错误输出(stderr)和/或文件，还可以写入其他输出流，并且可以在程序运行时对其进行影响。
 * 需要与log一起使用。
 * 日志等级：
 *      Trace < Debug < Info < Warn < Error
 */

#[cfg(test)]
mod flexi_test {
    use std::io::Result;

    use flexi_logger::{DeferredNow, FileSpec, LogSpecification, Logger, WriteMode};
    use log::{Record, info};

    #[test]
    fn test_1() {
        // 初始化，日志级别是info
        flexi_logger::init();
        /*  上述代码本质如下：
           Logger::try_with_env_or_str("info")
               .unwrap_or_else(|_e| Logger::with(LogSpecification::info()))
               .log_to_stderr()
               .start()
               .ok();
        */
        log::info!("info");
        log::debug!("debug"); // 不会输出
        log::trace!("trace"); // 不会输出
    }

    #[test]
    fn test_2() {
        // 初始化，设置全局的Log Level为debug
        flexi_logger::Logger::try_with_str("debug")
            .unwrap()
            .start()
            .unwrap();
        log::info!("info");
        log::trace!("trace"); // 不会输出
        log::debug!("debug");
    }

    // 输出到文件
    #[test]
    fn log_to_file() {
        /*  WriteMode可以取值：
               1、Direct： 每条日志直接写入输出目标，无缓冲，实时输出，频繁IO操作。
               2、Buffer：使用缓冲区，默认8KB，缓冲区满后自动刷新，减少了IO操作，程序崩溃会丢失日志。
               3、BufferAndFlush：使用缓冲区，每次日志写入后都尝试刷新，比Direct性能好，比Buffer可靠。
               4、Never：从不主动刷新缓冲区，依赖操作系统自动刷新
               5、Auto：自动选择最佳模式
        */
        let log_handle = Logger::try_with_str("info")
            .unwrap()
            .log_to_file(
                FileSpec::default()
                    .directory("src/logs")
                    .basename("app")
                    .suffix("log"),
            )
            .write_mode(WriteMode::Direct)
            .start()
            .unwrap();
        info!("info");

        log_handle.flush(); //当WriteMode取值为BufferAndFlush时需要手动flush
    }

    // 输出到控制台
    #[test]
    fn log_to_console() {
        Logger::try_with_str("info")
            .unwrap()
            .log_to_stdout()
            .write_mode(WriteMode::Direct)
            .start()
            .unwrap();

        info!("info");
    }

    // 同时将日志输出到文件和控制台
    #[test]
    fn log_to_file_and_console() {
        Logger::try_with_str("info")
            .unwrap()
            .log_to_file(
                FileSpec::default()
                    .directory("src/logs") // 日志文件所在的目录
                    .discriminant("abc") // 日志文件名中包含的标识符，会拼接在baseName后面
                    .basename("app") // 日志文件名前缀
                    .suffix("log") // 日志文件名后缀
                    .suppress_timestamp(), // 日志名不包含日期
            )
            .duplicate_to_stdout(flexi_logger::Duplicate::All)
            .write_mode(WriteMode::Direct)
            .start()
            .unwrap();

        info!("abc");
    }

    // 设置日志格式进行输出
    #[test]
    fn format_log_to_console() {
        Logger::try_with_str("info")
            .unwrap()
            .log_to_stdout()
            .write_mode(WriteMode::Direct)
            .format(file_log_format)
            .start()
            .unwrap();

        log::info!("info");
    }

    fn file_log_format(
        w: &mut dyn std::io::Write,
        now: &mut DeferredNow,
        record: &Record,
    ) -> Result<()> {
        write!(
            w,
            "[{}][{}][{}][{}:{}] - {}",
            now.now().format("%Y-%m-%d %H:%M:%S%.3f"), // 时间戳
            record.level(),                            // 日志级别
            record.module_path().unwrap_or("<unkonwn>"), // 模块路径
            record.file().unwrap_or("<unkonw>"),       // 文件名
            record.line().unwrap_or(0),                // 行号
            &record.args()                             // 日志内容
        )
    }

    // 输出到文件中的日志和控制台中的日志格式不同
    #[test]
    fn file_format_and_console_format() {
        let log_specification = LogSpecification::builder()
            .default(log::LevelFilter::Info)
            .build();
        Logger::with(log_specification)
            .format_for_files(file_log_format)
            .format_for_stdout(console_log_format)
            .log_to_file(FileSpec::default().directory("src/logs"))
            .duplicate_to_stdout(flexi_logger::Duplicate::All)
            .start()
            .unwrap();

        info!("This is a test log message.");
    }

    fn console_log_format(
        w: &mut dyn std::io::Write,
        _now: &mut DeferredNow,
        record: &Record,
    ) -> Result<()> {
        write!(w, "{}", &record.args())
    }
}
