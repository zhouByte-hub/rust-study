/** 用于 Rust 应用的分层配置系统
 *  支持的文件：
 *      1、ini - 添加对读取 INI 文件的支持
 *      2、json - 添加对读取 JSON 文件的支持
 *      3、yaml - 添加了读取 YAML 文件的支持
 *      4、toml - 添加了读取 TOML 文件的支持
 *      5、ron - 添加了读取 RON 文件的支持
 *      6、json5 - 添加了读取 JSON5 文件的支持
 */

#[cfg(test)]
mod test {

    use config::{Config, ConfigError};

    #[test]
    fn toml_config() -> Result<(), ConfigError> {
        // let value: TomlConfig = Config::builder().add_source(config::File::with_name("src/serialize/data/config.toml")).build()?;
        let value = Config::builder()
            .add_source(config::File::with_name("src/serialize/data/config.toml"))
            .build()?;
        println!("{:?}", value);
        Ok(())
    }

    #[test]
    fn ini_config() -> Result<(), ConfigError> {
        let value = Config::builder()
            .add_source(config::File::with_name("src/serialize/data/ini_test.ini"))
            .build()?;
        println!("{:?}", value);
        Ok(())
    }

    #[test]
    fn json_config() -> Result<(), ConfigError> {
        let value = Config::builder()
            .add_source(config::File::with_name("src/serialize/data/json_data.json"))
            .build()?;
        println!("{:?}", value);
        Ok(())
    }

    #[test]
    fn yaml_config() -> Result<(), ConfigError> {
        let value = Config::builder()
            .add_source(config::File::with_name("src/serialize/data/json_data.json"))
            .build()?;
        println!("{:?}", value);
        Ok(())
    }
}
