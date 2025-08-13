/**
 * 通过环境变量配置的log日志实现,可通过环境变量进行配置的日志记录器。
 * env_logger 本身不直接支持输出到文件，它默认只输出到控制台（stdout/stderr）。
 */

#[cfg(test)]
mod env_test{
    use log::{error, info};
    use std::io::Write;

    #[test]
    fn env_test_1(){
        // env_logger::init() 的作用是把 env_logger 作为日志后端注册到 log crate 的全局系统中，并根据环境变量配置日志的过滤和输出行为。
        env_logger::init();  
        info!("info");   // 不会输出
        error!("error"); // init()默认的日志级别是error
    }

    #[test]
    fn env_test_2(){
        // 初始化日志，设置日志级别
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();
        
        info!("abc");  
    }


    #[test]
    fn env_test_3(){
        env_logger::Builder::from_default_env().format(|buf, record| {
             writeln!(buf,
                "{} [{}] {}: {}",
                // chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                "time",
                record.level(),
                record.target(),
                record.args()
            )
        }).filter_level(log::LevelFilter::Info).init();

        info!("abc");
    }
}