use serde::{Deserialize, Serialize};

/**
 * toml = "0.9.5"
 *
* 一个兼容 serde 的 TOML 解析库，也就是说toml库是serde的一个实现库。
* TOML 本身是一种简单、易用且可读的配置格式，TOML就是一个增强版的ini。
*
* 格式如下：
* [package]
* name = "toml"
*
* [dependencies]
* serde = "1.0"
*
* TOML支持的类型如下：
* pub enum Value {
*     String(String),
*     Integer(i64),
*     Float(f64),
*     Boolean(bool),
*     Datetime(Datetime),
*     Array(Array),
*     Table(Table),
* }
* 而ini就只支持单一的格式String，如果碰到其他的就需要手动转换 
*/

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    age: u8,
    email: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    ip: String,
    port: Option<u16>,
    keys: Keys,
}

#[derive(Debug, Serialize, Deserialize)]
struct Keys {
    github: String,
    travis: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TomlConfig {
    database: Database,
    dependencies: Dependency,
    projects: Vec<Project>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Database {
    url: String,
    username: String,
    password: String,
    table: String,
    index: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct Dependency {
    package: Vec<String>,
    versions: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Project {
    name: String,
    version: String,
}

#[cfg(test)]
mod toml_test {
    use std::fs;
    use std::path::PathBuf;

    use crate::serialize::toml_test::Config;
    use crate::serialize::toml_test::TomlConfig;
    use crate::serialize::toml_test::User;

    #[test]
    fn write_test() {
        let user = User {
            username: String::from("张三"),
            age: 12,
            email: String::from("dayu-sec@dy.com"),
        };
        let value = toml::to_string(&user).unwrap();
        println!("{}", value);
    }

    #[test]
    fn read_test() {
        let config: Config = toml::from_str(
            r#"
            ip = '127.0.0.1'

            [keys]
            github = 'xxxxxxxxxxxxxxxxx'
            travis = 'yyyyyyyyyyyyyyyyy'
        "#,
        )
        .unwrap();
        println!("{}", config.ip);
    }

    #[test]
    fn read_config_test() {
        let content = fs::read_to_string(PathBuf::from("src/serialize/config.toml")).unwrap();
        let config: TomlConfig = toml::from_str(&content).unwrap();

        println!("{}", config.database.url);
    }
}
