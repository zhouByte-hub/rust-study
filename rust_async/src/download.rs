use std::{path::PathBuf, sync::Arc};
use tokio::sync::{Mutex, oneshot};
use tokio::task::yield_now;
use tokio::{io::Error, sync::mpsc};

// Tokio 任务是一个异步的绿色线程，它们通过 tokio::spawn 进行创建，该函数会返回一个 JoinHandle 类型的句柄，调用者可以使用该句柄跟创建的任务进行交互
pub async fn download(path: String) -> Result<PathBuf, Error> {
    /*
     * 执行任务的线程未必是创建任务的线程，任务完全有可能运行在另一个不同的线程上，而且任务在生成后，它还可能会在线程间被移动
     * 任务在 Tokio 中远比看上去要更轻量，例如创建一个任务仅仅需要一次 64 字节大小的内存分配
     * 当使用 Tokio 创建一个任务时，该任务类型的生命周期必须是 'static。意味着，在任务中不能使用外部数据的引用
     *
     * 默认情况下，变量并不是通过 move 的方式转移进 async 语句块， v 变量的所有权依然属于 main 函数因为任务内部的 println! 是通过借用的方式使用了 v，但是这种借用并不能满足 'static 生命周期的要求
     * 在报错的同时，Rust 编译器还给出了相当有帮助的提示：async 语句块使用 move 关键字，这样就能将 v 的所有权从 main 函数转移到新创建的任务中
     * 但 move 有一个问题，一个数据只能被一个任务使用。这时，Arc 起作用了，它还是线程安全的
     */
    let result = tokio::spawn(async {
        // 模拟下载操作代码省略...
        PathBuf::from(path)
    });
    Ok(result.await?)
}

/**
 * 在使用 Tokio 编写异步代码时，一个常见的错误无条件地使用 tokio::sync::Mutex
 * 而真相是 Tokio 提供的异步锁只应该在跨多个 .await 调用时使用，而且 Tokio 的 Mutex 实际上内部使用的也是 std::sync::Mutex
 * 锁如果在多个 .await 过程中持有，应该使用 Tokio 提供的锁，原因是 .await的过程中锁可能在线程间转移，若使用标准库的同步锁存在死锁的可能性
 * 锁竞争不多的情况下，使用 std::sync::Mutex
 *
 * tokio::spawn 生成的任务必须实现 Send 特征，因为当这些任务在 .await 执行过程中发生阻塞时，Tokio 调度器会将任务在线程间移动
 */
pub async fn lock_test() -> Result<(), Error> {
    let lock = Arc::new(Mutex::new(5));

    for _i in 0..10 {
        let temp_lock = lock.clone();
        tokio::spawn(async move {
            let mut value = temp_lock.lock().await;
            *value += 1;
        })
        .await?;
    }

    Ok(())
}

/**
 * 通道的缓冲队列长度是 32，意味着如果消息发送的比接收的快，这些消息将被存储在缓冲队列中，一旦存满了 32 条消息，使用send(...).await的发送者会进入睡眠，直到缓冲队列可以放入新的消息(被接收者消费了)
 * 可以使用 clone 方法克隆多个发送者，但是接收者无法被克隆，因为通道是 mpsc 类型
 * 当所有的发送者都被 Drop 掉后(超出作用域或被 drop(...)函数主动释放)，就不再会有任何消息发送给该通道,此时 recv 方法将返回 None，也意味着该通道已经被关闭
 *
 */
pub async fn channel_message() -> Result<(), Error> {
    let (tx, mut rx) = mpsc::channel(32);

    let clone_tx = tx.clone();
    let hello_handle = tokio::spawn(async move {
        let _ = tx.send("hello").await;
    });

    let world_handle = tokio::spawn(async move {
        let _ = clone_tx.send("world").await;
    });

    hello_handle.await?;
    world_handle.await?;

    if let Some(message) = rx.recv().await {
        println!("{message}");
    }
    Ok(())
}

/**
 * 使用 oneshot 消息通道，因为它针对一发一收的使用类型做过特别优化，且特别适用于此时的场景：接收一条从管理任务发送的结果消息
 * 使用方式跟 mpsc 很像，但是它并没有缓存长度，因为只能发送一条，接收一条
 * 还有一点不同：无法对返回的两个句柄进行 clone
 */
pub async fn oneshot_message() -> Result<(), Error> {
    let (tx, rx) = oneshot::channel();

    let handle = tokio::spawn(async move {
        // 往 oneshot 中发送消息时，并没有使用 .await，原因是该发送操作要么直接成功、要么失败，并不需要等待
        // 当 oneshot 的接受端被 drop 后，继续发送消息会直接返回 Err 错误，它表示接收者已经不感兴趣
        let _ = tx.send("hello world");
    });

    handle.await?;

    let a: &'static str = rx.await.unwrap();
    Ok(())
}

/**
 * 有时一个异步任务可能会占用较多的执行时间，导致其他任务无法及时得到执行。
 * tokio::task::yield_now 函数的作用就是让当前正在执行的异步任务主动让出执行权，将控制权交还给 tokio 运行时，这样运行时就可以调度其他等待的任务执行。
 * 当其他任务执行一段时间后，运行时会再回来继续执行这个让出执行权的任务。
 */
pub async fn yield_test() -> Result<(), Error> {
    let handle_1 = tokio::spawn(async {
        for i in 0..10 {
            if i == 5 {
                // 可以在适当的位置调用 yield_now().await 让出执行权，让其他任务有机会执行
                // 在多个异步任务协作的场景中，有时需要控制任务的执行顺序，通过调用 yield_now().await 可以实现任务之间的协作,确保各个任务按预期顺序执行
                yield_now().await;
            }
        }
    });

    let handle_2 = tokio::spawn(async {
        for x in 10..15 {
            println!("x = {x}");
        }
    });

    let _ = tokio::join!(handle_1, handle_2);
    Ok(())
}
