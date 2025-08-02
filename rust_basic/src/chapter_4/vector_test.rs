
/**
 * Vector是Rust中的动态数组，动态数组允许你存储多个值，这些值在内存中一个紧挨着另一个排列，因此访问其中某个元素的成本非常低。动态数组只能存储相同类型的元素
 * 数组的大小是可变的，当旧数组的大小不够用时，Rust 会重新分配一块更大的内存空间，然后把旧数组拷贝过来。
 */
pub fn test_1(){
    // 创建Vec对象
    let _list: Vec<i32>  = Vec::new();

    let mut a = Vec::new();
    a.push(12); // 当创建了Vec后，可以插入数据来确定Vec的类型

    // 可以使用宏来创建并赋值
    let mut _b = vec![1,2,3,4,5];

}


pub fn test_2(){
    let list = vec![1,2,3,4];
    // 获取元素
    let _a = list[0];
    let _b = list.get(1);

    /* 下标与get的区别
        1、使用下标可以很高效的获取到元素，但是当下标超过length时就会报下标越界的异常，导致程序终止。
        2、使用get的方式可以避免下标越界异常，当超过范围时获取的就是None，随之就不会像下标那样高效
     */
}


// 数组的遍历
pub fn test_3(){
    let mut list = vec![1,2,3,4,5];
    for item in &list{  // 得到的是不可变借用
        println!("{}", item);
    }

    for item in &mut list{
        *item += 1; // 使用引用进行操作时，需要进行解引用获取真的值
    }
}


// vector的排序
/**
 * 这个所谓的 非稳定 并不是指排序算法本身不稳定，而是指在排序过程中对相等元素的处理方式。在 稳定 排序算法里，对相等的元素，不会对其进行重新排序。而在 不稳定 的算法里则不保证这点。
 * 非稳定 排序的算法的速度会优于 稳定 排序算法，同时，稳定 排序还会额外分配原数组一半的空间。
 */
pub fn test_4(){
    let mut list = vec![1,3,2,4,7,5,6];
    list.sort(); // 稳定排序
    list.sort_unstable(); // 非稳定排序

    // 自定义稳定排序
    list.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // 自定义非稳定排序
    list.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
}