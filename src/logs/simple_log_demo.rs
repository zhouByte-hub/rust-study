/**
 *  Rust 实现的简单日志，支持本地文件或标准输出写入。
 */

#[cfg(test)]
mod simple_log_demo {

    use simple_log::{debug, info, LogConfigBuilder};


    #[test]
    fn test_1(){
        simple_log::quick!("info");
        info!("abc");
    }


    #[test]
    fn test_2(){
        let log_config = LogConfigBuilder::builder()
            .path("src/logs/simple.log")
            .size(10)
            .roll_count(8)
            .time_format("%Y-%m-%d %H:%M:%S.%f")
            .level("debug")
            .unwrap()
            .output_console()
            .output_file()
            .build();
        simple_log::new(log_config).unwrap();

        info!("abc");
        debug!("debug");
    }
}