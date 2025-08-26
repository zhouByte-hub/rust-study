/**
 * ringbuf = "0.4.8"
 *
 * ringbuf 是一个为 Rust 设计的无锁（lock-free）、单生产者单消费者（SPSC）的环形缓冲区（Ring Buffer）库。
 * 它旨在为需要在两个线程（一个生产者，一个消费者）之间进行极高性能、低延迟数据传输的场景提供优化的解决方案。
 *
 * 核心功能：提供一个固定容量的环形缓冲区（FIFO 队列）。
 * 在 SPSC 场景下，其性能通常优于通用的 MPMC 队列（如 crossbeam-queue 的 SegQueue），因为它可以做出更强的假设并进行更激进的优化。
 * 缓冲区的大小在创建时确定，无法动态增长。
 *
 * 适用场景：
 *      两个线程之间的高速数据流<br>例如：音频处理、实时数据采集、高性能日志、游戏引擎中的特定模块通信。
 */
#[cfg(test)]
mod ringbut_test {
    use ringbuf::traits::{Consumer, Producer, Split};
    use std::thread;

    #[test]
    fn test_1() {
        let ringbuf = ringbuf::HeapRb::<i32>::new(10);
        let (mut producer, mut consumer) = ringbuf.split();

        thread::spawn(move || {
            for i in 0..20 {
                match producer.try_push(i) {
                    Ok(_) => println!("{} push success", i),
                    Err(e) => println!("push failed = {}", e), // 超过容量会失败
                }
            }
        })
        .join()
        .unwrap();

        while let Some(item) = consumer.try_pop() {
            println!("item = {}", item);
        }
    }
}
