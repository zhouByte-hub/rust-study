/**
 * mockall = "0.13.1"
 *
 * 一个强大的 Rust 模拟对象库。
 *
 * 模拟对象是软件单元测试的一种强大技术。
 * 在软件测试中，Mock（模拟）对象是用来模拟真实对象行为的替身。
 * 当你测试一个模块（A）时，如果这个模块依赖于另一个模块（B），但模块 B 难以在测试环境中使用（例如，它涉及数据库操作、网络请求、复杂的计算或者还不存在），你就可以用一个 Mock 对象来替代真实的模块 B。
 * Mockall 结合了以前设计的最佳元素，使其具有丰富的功能集和简洁的易用界面。Mockall 是用 100%安全稳定的 Rust 编写的。
 *
 * 它的主要功能包括：
 * 1. 自动生成模拟实现：你可以为 trait 或 struct 自动生成模拟实现，而无需手动编写。
 * 2. 配置模拟行为：你可以配置模拟对象的行为，例如返回特定值、抛出错误或记录调用信息。
 * 3. 验证调用：你可以验证模拟对象是否按照预期被调用，以及调用次数和参数。
 * 4. 支持异步代码：Mockall 支持异步代码的模拟，包括 async/await 函数和 Future 类型。
 *
 */
#[cfg(test)]
mod mack_test {
    use httpmock::MockServer;
    use mockall::automock;
    use std::error::Error;

    /**
     * #[cfg_attr(test, automock)] 是一个 条件性属性
     * 只有在特定的编译配置下，才会应用某个属性。
     * 这里的 test 是一个编译配置，只有在测试环境下才会应用 automock 属性。
     */
    #[cfg_attr(test, automock)]
    trait HttpClient {
        fn post(&self, url: &str) -> Result<String, Box<dyn Error>>;
    }

    #[derive(Debug)]
    struct Database<T: HttpClient> {
        client: T,
    }

    impl<T: HttpClient> Database<T> {
        fn new(client: T) -> Self {
            Self { client }
        }
        fn fetch(&self) -> String {
            let result = self.client.post("http://www.baidu.com").unwrap();
            result
        }
    }

    #[test]
    fn test_1() {
        // 1、创建一个Mock对象,
        let mut mock_client = MockHttpClient::new(); // 类型名是 Mock + 原Trait名
        mock_client
            .expect_post() // 期望调用的方法
            .times(1) // 期望调用的次数
            .withf(|url| url == "http://www.baidu.com") // 期望调用的参数
            .returning(|result| Ok(result.to_string())); // 期望返回的结果

        let databse = Database::new(mock_client);

        let result = databse.fetch();
        println!("{:?}", result); // http://www.baidu.com
    }

    #[tokio::test]
    async fn http_mock() {
        // 启动一个模拟服务器
        let server = MockServer::start();
        // 设置 mock 期望
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/api/username");
            then.status(200)
                .header("content-type", "application/json")
                .body("zhangsan");
        });

        let _content = server.url("/api/username"); // http://127.0.0.1:53817/api/username
        // let response = reqwest::get(&content)
        //     .await
        //     .unwrap();
        // let body = response.text().await.unwrap();
        // println!("{:?}", body); // zhangsan
    }
}
