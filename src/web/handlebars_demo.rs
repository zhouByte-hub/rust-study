/**
* handlebars = "6.3.2"
*
* 这是 Rust 的一个模板引擎库，它实现了 Handlebars.js 的大部分功能。
* 它允许你定义模板（通常是 HTML、配置文件或其他文本格式），然后将数据（通常是一个 Rust 结构体或 serde_json::Value）填充到这些模板中，生成最终的文本输出。

*/

#[cfg(test)]
mod handlebars_test {
    use handlebars::Handlebars;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct User {
        username: String,
        age: u8,
    }

    #[test]
    fn string_template_test() {
        let user = User {
            username: "zhangsan".to_string(),
            age: 12,
        };

        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_string("show_user", "{{username}} is {{age}} years old.")
            .unwrap();

        let render = handlebars.render("show_user", &user).unwrap();
        println!("{}", render);
    }

    #[test]
    fn file_template_test() {
        let user = User {
            username: "zhangsan".to_string(),
            age: 12,
        };

        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_file("show_user", "src/web/template/user.html")
            .unwrap();

        let render = handlebars.render("show_user", &user).unwrap();
        println!("{}", render);
    }
}
