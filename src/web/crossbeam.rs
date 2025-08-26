/**
 * crossbeam-queue = "0.3.12"
 *
 * crossbeam-queue 是 Rust 语言中一个非常重要的并发编程库，它提供了高性能、无锁（lock-free）的线程安全队列数据结构。
 * 简单来说，它允许你在多个线程之间安全、高效地传递数据，而无需使用传统的互斥锁（Mutex），从而在高并发场景下获得更好的性能。
 *
 * 核心特性：
 *      1、无锁（lock-free）：crossbeam-queue 内部使用了无锁算法，避免了传统锁机制的性能瓶颈，如死锁、饥饿等问题。
 *      2、高性能：由于无锁特性，crossbeam-queue 在高并发场景下具有出色的性能表现，能够处理 millions of operations per second（MOPs）。
 *      3、线程安全：所有队列操作都是线程安全的，无需额外的同步机制。
 *      4、多种队列类型：提供了多种队列类型，如 MPMC（多生产者多消费者）、MPMC 无界（无边界）、MPMC 有界（有边界）等。
 * 两个核心队列：
 *      1、segQueue<T>：多生产者多消费者，无锁，无边界；基于分段形成的环形缓冲区，性能优于ArrayQueue，尤其是在共享资源高竞争情况下。
 *      2、ArrayQueue<T>：多生产者多消费者，无锁，有边界，基于固定大小的环形缓冲区（数组），容量在创建时固定；当容量满了只会再push就会返回Err。
 */
#[cfg(test)]
mod crossbeam_test {
    use std::{
        sync::{Arc, mpsc::channel},
        thread,
    };

    use crossbeam_queue::{ArrayQueue, SegQueue};

    #[test]
    fn test_1() {
        let queue = Arc::new(SegQueue::new()); // 创建一个队列，用于生产者和消费者之间传递数据。

        let queue_copy = Arc::clone(&queue);
        let producer = thread::spawn(move || {
            for item in 1..10 {
                queue.push(item);
            }
        });
        producer.join().unwrap();

        let consumer = thread::spawn(move || {
            while let Some(message) = queue_copy.pop() {
                println!("message = {}", message);
            }
        });
        consumer.join().unwrap();
    }

    #[test]
    fn test_2() {
        let queue = Arc::new(ArrayQueue::new(10));

        let queue_clone = Arc::clone(&queue);
        thread::spawn(move || {
            for item in 1..20 {
                match queue.push(item) {
                    Ok(_) => println!("{} push success", item),
                    Err(e) => println!("push failed = {}", e), // 11 ~ 19都会走这个分支
                }
            }
        })
        .join()
        .unwrap();

        thread::spawn(move || {
            while let Some(item) = queue_clone.pop() {
                println!("item = {}", item);
            }
        })
        .join()
        .unwrap();
    }

    #[test]
    fn test_3() {
        let (sender, receiver) = channel();

        thread::spawn(move || {
            for i in 0..10 {
                sender.send(i).unwrap();
            }
        })
        .join()
        .unwrap();

        for i in receiver {
            println!("i = {}", i);
        }
    }
}
