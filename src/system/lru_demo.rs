/**
 * lru = "0.16.1"
 * 
 * LRU（Least Recently Used）是"最近最少使用"的缩写，是一种广泛应用于缓存管理的页面置换算法。
 * 它的核心思想是：如果数据最近被访问过，那么将来被访问的几率也更高。因此，当缓存空间不足时，LRU算法会优先淘汰最近最少使用的数据。
 * 
 * LRU 缓存的实现。该缓存支持 put 、 get 、 get_mut 和 pop 操作，所有操作的复杂度均为 O(1)。
 */

#[cfg(test)]
mod lru_test{
    use std::num::NonZeroUsize;

    use lru::LruCache;


    #[test]
    fn test(){
        let mut cache = LruCache::new(NonZeroUsize::new(2).unwrap());
        // 插入
        cache.put("name", "zhoubyte");
        cache.put("age", "12");

        // 查询
        assert_eq!(cache.get("name"), Some(&"zhoubyte"));
        assert_eq!(cache.get("age"), Some(&"12"));
        assert!(cache.get("sex").is_none());

        cache.put("address", "beijing");
        assert_eq!(cache.get("address"), Some(&"beijing"));

        println!("{}", cache.len());

        cache.iter().for_each(|(key,value)|{
            println!("{}:{}", key, value);
        });
    }
}