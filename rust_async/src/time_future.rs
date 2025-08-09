use std::{
    sync::{Arc, Mutex},
    task::{Poll, Waker},
    thread,
    time::Duration,
};

/**
 * 实现一个定时器的功能，在指定的时间后唤醒任务
 */

#[derive(Debug)]
pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedStatus>>,
}

#[derive(Debug)]
struct SharedStatus {
    completed: bool,
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            /*
               TimerFuture可在执行的任务间移动，这回导致过期的waker指向错误的任务。
               从而阻止了TimerFuture的正确唤醒。
            */
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedStatus {
            completed: false,
            waker: None,
        }));
        let temp = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut state = temp.lock().unwrap();
            // 发出信号，唤醒Future
            state.completed = true;
            if let Some(waker) = state.waker.take() {
                waker.wake();
            }
        });
        TimerFuture { shared_state }
    }
}

#[cfg(test)]
mod test {
    use std::{thread, time::Duration};

    use crate::time_future::TimerFuture;

    #[test]
    fn test() {
        let timer = TimerFuture::new(Duration::from_secs(2));
        // 调用.await来获取值
        println!("{:?}", timer);

        thread::sleep(Duration::from_secs(5));
    }
}
