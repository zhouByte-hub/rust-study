use std::cell::RefCell;
use std::sync::Arc;
use std::{sync::Barrier, thread, time::Duration};

/**
 * 并发(Concurrent) 是多个队列使用同一个咖啡机，然后两个队列轮换着使用（未必是 1:1 轮换，也可能是其它轮换规则），最终每个人都能接到咖啡。
 * 并行(Parallel) 是每个队列都拥有一个咖啡机，最终也是每个人都能接到咖啡，但是效率更高，因为同时可以有两个人在接咖啡
 */

fn test() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
        }
    });

    thread::sleep(Duration::from_secs(5));
}

fn test_2() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
        }
    });

    handle.join().unwrap(); // 等待线程结束

    println!("end");
}

// 线程屏障：可以使用Barrier让多个线程都执行到某个点后，才继续往下执行。
fn test_3() {
    let mut list = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));
    for _i in 0..6 {
        let b = barrier.clone(); // 每个线程都拥有一个Barrier的拷贝
        let handle = thread::spawn(move || {
            println!("hi number from the spawned thread!");
            b.wait(); // 等待所有线程都执行到这里
            println!("end");
        });
        list.push(handle);
    }

    for handle in list {
        handle.join().unwrap();
    }
}

// ThreadLocal：每个线程拥有自己的数据，线程之间不共享
fn test_4() {
    thread_local! {
        // 只能是静态变量
        static FOO: RefCell<String> = RefCell::new(String::new());
    }

    for _i in 0..10 {
        thread::spawn(|| {
            FOO.with(|f| {
                // 通过借用的方式，访问ThreadLocal中的数据
                *f.borrow_mut() = format!("abc");
            });
        });
    }
}
