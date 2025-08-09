use serde::{Deserialize, Serialize};

/**
 * serde_json是对serde的一种json方式的序列化和反序列化的方式
 *
 * serde_json = "1.0.142"
 */

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    age: u8,
}

#[allow(dead_code)]
impl User {
    pub fn new(username: &str, age: u8) -> Self {
        User {
            username: String::from(username),
            age,
        }
    }
}

#[cfg(test)]
mod test {

    use std::{fs, io::Write, path::PathBuf};

    use serde_json::Error;

    use crate::serialize::serde_json_test::User;

    #[test]
    fn test_1() -> Result<(), Error> {
        // 使用serde_json::to_string实现序列化
        let user = User::new("username", 12);
        let value = serde_json::to_string(&user)?;
        // serde_json::to_writer(writer, value)    // 将对象序列化到一个输出流中，File, TcpStream, Vec<u8>, BufWriter都是Writer的实现
        println!("{:?}", value);
        Ok(())
    }

    #[test]
    fn test_2() -> Result<(), Error> {
        // 使用serde_json::from_str实现反序列化
        let content = "{\"username\":\"username\",\"age\":12}";
        let value: User = serde_json::from_str(content)?;
        // serde_json::from_reader(rdr) // 从一个Read流中反序列化
        println!("{:?}", value);
        Ok(())
    }

    #[test]
    fn test_3() -> Result<(), Error> {
        // 序列化到文件
        let user = User::new("zhangsan", 12);
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(PathBuf::from(
                "E:\\project\\rust\\rust-study\\src\\serialize\\writer.json",
            ))
            .unwrap();
        serde_json::to_writer(&file, &user)?;
        file.flush().unwrap();
        Ok(())
    }

    #[test]
    fn test_4() -> Result<(), Error> {
        // 从文件反序列化成对象
        let file = fs::OpenOptions::new()
            .read(true)
            .open(PathBuf::from(
                "E:\\project\\rust\\rust-study\\src\\serialize\\writer.json",
            ))
            .unwrap();
        let user: User = serde_json::from_reader(&file)?;
        println!("{:?}", user);
        Ok(())
    }
}
