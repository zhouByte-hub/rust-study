/**
 * rust-ini = "0.21"
 *
 * ini文件的格式如下：section下其实是一个map，可以使用迭代器进行遍历
 * [section]
 * key = value
 * key = value
 * [section] * key = value
 */
#[cfg(test)]
mod ini_test {
    use std::{io::Result, path::PathBuf};

    use ini::Ini;

    #[test]
    fn write_test() -> Result<()> {
        let mut config = Ini::new();

        config
            .with_section(None::<String>)
            .set("company", "dayu-sec.com");
        config
            .with_section(Some("deployment"))
            .set("dev", "左文建")
            .set("deploy", "周海军");
        config.write_to_file(PathBuf::from("src/serialize/data/ini_test.ini"))?;
        Ok(())
    }

    #[test]
    fn read_test() {
        let config = Ini::load_from_file(PathBuf::from("src/serialize/ini_test.ini")).unwrap();
        // 方式一
        // let dev = config.section(Some("deployment")).unwrap().get("dev").unwrap();

        // 方式二
        let email = config
            .section(Some("deployment"))
            .and_then(|properties| {
                let value = properties.get("email").unwrap_or("dayu-sec");
                Some(value)
            })
            .unwrap();

        println!("{email}");
    }
}
