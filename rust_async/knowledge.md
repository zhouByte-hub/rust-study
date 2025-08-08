# 1、Rust并发编程
async编程是一种并发编程模型，允许在少数系统线程上运行大量的并发任务，通过async/await语法，看起来和同步编程差不多。

- Future：是惰性的，只有poll时才能取得进展，被丢弃的future就无法取得进展。
- async：零成本操作，使用async,可以无需堆内存分配和动态调度，对性能大好，且允许在受限的环境中使用async。
- 不提供内置运行时
- 单线程、多线程均支持，但缺点不同

## 1.1、其他并发模型
- OS线程
  - 无需改变编程模型，线程间同步困难，性能开销大。
  - 线程池可以降低一些成本，但是难以支撑大量IO绑定的工作。
- Event-driven编程
  - 与回调函数一起使用，可以高效。
  - 非线性的控制流，数据流和错误传播难以追踪。
- Coroutines
  - 类似线程，无需改变编程模型。
  - 类似async，支持大量任务。
  - 抽象掉了底层细节（对系统编程、自定义运行时的实现很重要）
- Actor模型
  - 将所有并发计算划分为Actor，消息通信易出错。
  - 可以有效的实现actor模型，但许多实际问题没解决（控制流、重试逻辑）
  
## 1.2、async和线程
（1）OS线程
- 适用于少量任务，有内存和CPU开销，且线程生成和线程间切换非常昂贵。
- 线程池可以降低一些成本。
- 允许重用同步代码，代码无需大改，无需特定编程模型。
- 有些系统支持修改线程优先级。

（2）Async
- 显著降低内存和CPU开销。
- 同等条件下，支持比线程多几个数量级的任务（少量线程支撑大量任务）
- 可执行文件大（需要生成状态机，每个可执行文件捆绑一个异步运行时）

虽然Rust本身就支持async编程，但很多应用依赖于社区的库：
- 标准库提供了最基础的特性、类型和功能，例如：Future trait。
- async/await语法直接被Rust编译器支持。
- futures crate提供了许多实用类型、宏和函数，他们可以用于任何异步应用程序。
- 异步代码、IO和任务生成的执行由async runtimes提供，例如Tokio和async-std。大多数async应用程序和一些async crate都依赖于特定的运行时。

<b style="color:red">async把一段代码转化为一个实现了Future trait的状态机，虽然在同步方法中调用阻塞函数会阻塞整个线程，但阻塞的Future将放弃对线程的控制，从而允许其他Future来运行。</b>

## 1.3、futures
（1）Future trait
- Future trait是Rust Async编程的核心。
- Future是一种异步计算，他可以产生一个值。
- 实现了Future的类型表示目前可能还不可用的值。
```rust
// 下面是一个简化版的Future

trait SimpleFuture {
    // Future返回的可用的值的类型
    type Output;
    // 调用poll方法就会驱动Future尽可能的向着完成来执行；如果Future还无法完成，就会返回Pending，并当Future可以取得更多进展时就会调用wake回调函数。
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

/**
 * 如果返回的是Ready表示Future已经结束
 * 如果返回的是Pending表示Future还没有结束
 **/
enum Poll<T> {
    Ready(T),
    Pending
}
```

（2）wake()函数
- 当wake()函数被调用时，执行器将将驱动future再次调用poll函数，以便futur能取得更多进展。
- 没有wake函数，执行器就不知道特定的future何时能取得进展（就得不断的poll，效率就会非常低下）
- 通过wake()函数，执行器就确切的知道哪些Future已经准备好进行Poll()的调用。
```rust
pub struct SocketRead<'a> {
    socket: &'a Socket
}

impl impleFuture for SocketRead<'_> {
    type Output = Vec<u8>;

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if self.socket.has_data_to_read() {
            Poll::Ready(self.socket.read_buf())
        } else{
            self.socket.set_readable_callback(wake);
            Poll::Pending
        }
    }
}
```

## 1.4、Tokio
Tokio 是 Rust 编程语言的异步运行时。它提供了编写网络应用程序所需的构建块。它提供了针对各种系统的灵活性，从具有数十个内核的大型服务器到小型嵌入式设备。

优势：
- Tokio 速度很快 ，建立在 Rust 编程语言之上，而 Rust 编程语言本身就很快。这是本着 Rust 的精神完成的，目的是您不应该能够通过手动编写等效代码来提高性能
- Tokio 是使用 Rust 构建的，Rust 是一种使每个人都能构建可靠、高效的软件的语言。
- 借助 Rust 的异步/等待功能，编写异步应用程序的复杂性已大大降低。与 Tokio 的实用程序和充满活力的生态系统相结合，编写应用程序变得轻而易举。
- Tokio 提供了运行时的多种变体。从多线程、 节省工作量的运行时到轻量级的单线程运行时，应有尽有。这些运行时中的每一个都带有许多旋钮，允许用户根据自己的需要进行调整。

