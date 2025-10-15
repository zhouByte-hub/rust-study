/**
 * getset = "0.1.6"
 *
 * 用于生成字段最基本的 getter 和 setter 的程序宏。
 */
#[allow(dead_code)]
#[cfg(test)]
mod getset_test {
    use getset::{CloneGetters, CopyGetters, Getters, MutGetters, Setters, WithSetters};

    #[derive(Getters, Setters, WithSetters, MutGetters, CopyGetters, CloneGetters, Default)]
    struct User {
        #[getset(get, set, get_mut, set_with)]
        username: String,

        #[getset(get, set, get_mut, set_with)]
        age: u8,

        #[getset(get, set, get_mut, set_with)]
        address: String,
    }

    fn test() {
        let mut user = User {
            username: "zhangsan".to_string(),
            age: 12,
            address: "beijing".to_string(),
        };
        user.set_age(13);
    }
}
