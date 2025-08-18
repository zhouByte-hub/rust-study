/**
 * 在使用tokio的时候，它内部是维护了两个线程池：
 *      1、阻塞线程池      -    tokio::task::spawn_blocking
 *      2、非阻塞线程池    -    tokio::spawn
 */

#[cfg(test)]
mod spawn_test {

    #[tokio::test]
    async fn test_1() {
        tokio::spawn(async move {
            let mut result = 0;
            for i in 0..100 {
                result += i;
            }
            println!("{}", result);
        })
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn test_2() {
        tokio::task::spawn_blocking(move || {
            let mut result = 0;
            for i in 0..100 {
                result += i;
            }
            println!("{}", result);
        })
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn wait_test() {
        // 对于已经知道个数的任务，可以使用tokio::join!
        // tokio::join!(test_1(), test_2());

        // 对于不知道个数的任务，可以使用JoinSet
        let mut join_set = tokio::task::JoinSet::new();
        join_set.spawn(async move {
            test_1();
            test_2();
            // Ok::<(), Box<dyn std::error::Error + Send + Sync>> (()) // 显式返回 Result
        });
        while let Some(res) = join_set.join_next().await {
            if let Err(e) = res {
                println!("{:?}", e);
            }
        }
    }
}
