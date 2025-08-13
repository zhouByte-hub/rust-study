/**
 * pretty_env_logger 是 env_logger 的一个“美化版”，它提供了更美观、更易读的日志输出格式，特别适合在开发和调试时使用。
 */

#[cfg(test)]
mod pretty_test {
    use log::{warn, info, error};
    extern crate pretty_env_logger;

    #[test]
    fn test_1() {
        pretty_env_logger::init(); // init默认的日志级别也是error
        info!("such information"); // 不打印
        warn!("o_O"); // 不打印
        error!("much error"); // 打印
    }

    #[test]
    fn test_2() {
        pretty_env_logger::formatted_builder()
            .filter_level(log::LevelFilter::Info)   // 全局日志级别
            .filter_module("reqwest", log::LevelFilter::Debug)  // 模块日志级别
            .init();

        info!("info");
    }
}
