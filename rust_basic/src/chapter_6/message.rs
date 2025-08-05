use std::{sync::mpsc, thread};

/** 线程消息传递
 *  多发送者 -> 单接收者，多发送者 -> 多接收者
 *  注意点：
 *      1、tx,rx对应发送者和接收者，它们的类型由编译器自动推导: tx.send(1)发送了整数，因此它们分别是mpsc::Sender<i32>和mpsc::Receiver<i32>类型，
 *          需要注意，由于内部是泛型实现，一旦类型被推导确定，该通道就只能传递对应类型的值, 例如此例中非i32类型的值将导致编译错误。
 *      2、接收消息的操作rx.recv()会阻塞当前线程，直到读取到值，或者通道被关闭
 *      3、需要使用move将tx的所有权转移到子线程的闭包中
 * 
 * 发送者依旧遵循Rust所有权的原则：
 *      1、若值的类型实现了Copy特征，则直接复制一份该值，然后传输过去，例如之前的i32类型
 *      2、若值没有实现Copy，则它的所有权会被转移给接收端，在发送端继续使用该值将报错
 * 
 * 所有发送者被drop或者所有接收者被drop后，通道会自动关闭
 */

fn test_1(){
    // mpsc::channel得到的是一个元组，分别对应发送者和接收者
    let (tx, tr) = mpsc::channel();

    // 对于使用发送者和接收者来说，就需要将所有权移入子线程中。
    thread::spawn(move || {
        tx.send("abc").unwrap(); 
    });

    thread::spawn(move || {
        tr.recv().unwrap(); // recv会阻塞当前线程，可以使用try_recv()
        tr.try_recv().unwrap();    // 不会阻塞当前线程
    });
}

// 循环接受消息
fn test_2() {
    let (tx, tr) = mpsc::channel();
    thread::spawn(move || {
        tx.send("abc").unwrap();
    });

    for item in tr {
        println!("{}", item);
    }
}

// 同步通道
fn test_3() {
    /*
     * bound该值可以用来指定同步通道的消息缓存条数，当你设定为N时，发送者就可以无阻塞的往通道中发送N条消息，当消息缓冲队列满了后，
     * 新的消息发送将被阻塞(如果没有接收者消费缓冲队列中的消息，那么第N+1条消息就将触发发送阻塞)。
     */
    let (tr, tx) = mpsc::sync_channel(0);
    tr.send("abc").unwrap();
    tx.recv().unwrap();
}