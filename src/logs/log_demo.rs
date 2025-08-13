/**
 * log提供了一个单一的日志 API，它抽象了实际的日志实现。库可以使用这个 crate 提供的日志 API，而使用这些库的消费者可以选择最适合其使用场景的日志实现。
 * log crate 只是一个日志门面；它本身不提供日志的输出实现。，为了生成日志输出，可执行文件必须使用与 facade 兼容的日志记录器实现：
 *      1、env_logger
 *      3、pretty_env_logger
 *      4、flexi_logger
 */
#[cfg(test)]
mod log_test {

    #[test]
    fn test_1() {
        log::info!("info"); // 没有输出
        log::trace!("trace"); // 没有输出
    }
}
