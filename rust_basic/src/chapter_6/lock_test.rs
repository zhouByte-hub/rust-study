use std::{
    sync::{Arc, Mutex, RwLock},
    thread,
};

/**
 * 在多线程编程中，同步性极其的重要，当你需要同时访问一个资源、控制不同线程的执行次序时，都需要使用到同步性。
 * 在 Rust 中有多种方式可以实现同步性。在上一节中讲到的消息传递就是同步性的一种实现方式，例如我们可以通过消息传递来控制不同线程间的执行次序。还可以使用共享内存来实现同步性，
 * 例如通过锁和原子操作等并发原语来实现多个线程同时且安全地去访问一个资源。
 *
 * 共享内存可以说是同步的灵魂，因为消息传递的底层实际上也是通过共享内存来实现，两者的区别如下:
 *      1、共享内存相对消息传递能节省多次内存拷贝的成本
 *      2、共享内存的实现简洁的多
 *      3、共享内存的锁竞争更多
 * 消息传递适用的场景很多，我们下面列出了几个主要的使用场景:
 *      1、需要可靠和简单的(简单不等于简洁)实现时
 *      2、需要模拟现实世界，例如用消息去通知某个目标执行相应的操作时
 *      3、需要一个任务处理流水线(管道)时，等等
 * 而使用共享内存(并发原语)的场景往往就比较简单粗暴：需要简洁的实现以及更高的性能时。
 *
 * 总之，消息传递类似一个单所有权的系统：一个值同时只能有一个所有者，如果另一个线程需要该值的所有权，需要将所有权通过消息传递进行转移。而共享内存类似于一个多所有权的系统：多个线程可以同时访问同一个值。
 */

/** 互斥锁：Mutex
 * Mutex让多个线程并发的访问同一个值变成了排队访问：同一时间，只允许一个线程A访问该值，其它线程需要等待A访问完成后才能继续。
*/
fn test_1() {
    let value = Mutex::new(String::from("abc"));
    let rc = Arc::new(value); // 多线程环境下要使用线程安全的ARC

    for i in 0..10 {
        let temp_rc = rc.clone();
        thread::spawn(move || {
            let mut a = temp_rc.lock().unwrap();
            *a = format!("test_{i}");
        });
    }
    println!("value = {}", rc.lock().unwrap());
}

fn test_2() {
    let value = Mutex::new(5);
    let rc = Arc::new(value);

    for _i in 0..10 {
        let temp_rc = rc.clone();
        thread::spawn(move || {
            temp_rc.try_lock().unwrap(); // 使用try_lock方法尝试去获取一次锁，当获取不到就直接报错，不会发生阻塞
        });
    }
}

// 读写锁：同一时间允许有多个读，但是只能有一个写
fn test_3() {
    let lock = RwLock::new(5);
    let rc = Arc::new(lock);

    for _i in 0..10 {
        let temp_rc = rc.clone();
        let value = temp_rc.read().unwrap();
        println!("{}", *value);

        let value = temp_rc.read().unwrap();
        println!("{}", *value);
    }

    let mut a = rc.write().unwrap();
    *a += 1;
    println!("{}", *a);
}
