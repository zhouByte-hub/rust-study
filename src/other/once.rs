/**
 * once_cell = "1.21.3"
 *
 * once_cell 是 Rust 的一个非常流行的库，用于实现 一次性初始化 的全局变量和懒加载值。它的名字来源于 "once"（一次）和 "cell"（单元格），意思是"只能初始化一次的单元格"。
 */

#[cfg(test)]
mod once_test {
    use std::collections::HashMap;

    // sync是线程安全版本， unsync 是线程不安全版本
    use once_cell::sync::{Lazy, OnceCell};

    // 全局配置，需要在运行时初始化
    static CONFIG: OnceCell<HashMap<&'static str, &'static str>> = OnceCell::new();

    // 懒加载常量
    static DATA: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
        let mut hash_map = HashMap::new();
        hash_map.insert("username", "zhangsan");
        hash_map.insert("age", "12");
        hash_map
    });

    #[test]
    fn test_1() {
        // 第一次调用完成初始化并获取值
        let config = CONFIG.get_or_init(|| DATA.clone());
        println!("{}", config.get("username").unwrap());
    }
}
