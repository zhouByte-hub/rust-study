/**
 * ctrlc = "3.4.7"
 *
 * 用于在 Rust 程序中处理操作系统的信号（Signals），最常见的是处理用户按下 Ctrl+C 组合键（在 Unix/Linux/macOS 上对应 SIGINT 信号，在 Windows 上也有相应的中断处理）时的事件。
 */

#[cfg(test)]
mod ctrlc_demo_test {
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;
    use std::time::Duration;

    #[test]
    fn test() {
        // 创建一个共享的原子布尔值，用于标记程序是否应该退出
        let running = Arc::new(AtomicBool::new(true));

        let running_clone = Arc::clone(&running);
        // 设置中断处理程序，本质上就是开启一个线程去监听中断信号
        ctrlc::set_handler(move || {
            /* Ordering 的取值
             * SeqCst：顺序一致性（Sequentially Consistent），这是最严格的顺序一致性，确保所有线程都以一致的顺序看到所有操作的结果。
             * Relaxed：宽松顺序，不保证任何顺序，只是确保操作的可见性。
             * Release：释放顺序，确保当前线程的写操作对其他线程可见，但是不保证其他线程的操作顺序。
             * Acquire：获取顺序，确保当前线程的读操作在其他线程的写操作之前执行。
             * AcqRel：获取释放顺序，结合了 Acquire 和 Release 的语义。
             */
            running_clone.store(false, Ordering::SeqCst);
        })
        .unwrap();

        println!("设置中断处理程序成功");

        while running.load(Ordering::SeqCst) {
            println!("按 Ctrl+C 退出");
            std::thread::sleep(Duration::from_millis(1000));
        }
        println!("程序优雅退出");
    }
}
