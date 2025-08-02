use std::collections::HashMap;

/**
 * 和动态数组一样，HashMap 也是 Rust 标准库中提供的集合类型，但是又与动态数组不同，HashMap 中存储的是一一映射的 KV 键值对，
 * 并提供了平均复杂度为 O(1) 的查询方法，当我们希望通过一个 Key 去查询值时，该类型非常有用
 */

pub fn test_1(){
    let mut map = HashMap::new();
    map.insert("红宝石", 1);

    let list = vec![
        ("中国队", 1),
        ("美国队", 2)
    ];

    // 在使用collector的时候，需要给变量明确类型，才能让编译器进行推导
    let mut list_map: HashMap<_,_> = list.into_iter().collect();
    list_map.insert("巴拿马队", 3);

    println!("{}", list_map.get("中国队").unwrap());
}


#[cfg(test)]
mod test{
    use std::collections::HashSet;

    #[test]
    pub fn list_test(){
        // HashSet
        let _set: HashSet<i32> = HashSet::new();
    }

}