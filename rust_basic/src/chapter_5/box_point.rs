/** Box智能指针
 * Box<T> 允许你将一个值分配到堆上，然后在栈上保留一个智能指针指向堆上的数据。
 * 堆上内存则是从低位地址向上增长，堆内存通常只受物理内存限制，而且通常是不连续的，因此从性能的角度看，栈往往比堆更高效。
 *      1、小型数据，在栈上的分配性能和读取性能都要比堆上高
 *      2、中型数据，栈上分配性能高，但是读取性能和堆上并无区别，因为无法利用寄存器或 CPU 高速缓存，最终还是要经过一次内存寻址
 *      3、大型数据，只建议在堆上分配和使用
 * 总之，栈的分配速度肯定比堆上快，但是读取速度往往取决于你的数据能不能放入寄存器或 CPU 高速缓存。 因此不要仅仅因为堆上性能不如栈这个印象，就总是优先选择栈，导致代码更复杂的实现。
 */

/** Box 是简单的封装，除了将值存储在堆上外，并没有其它性能上的损耗，但是功能单一。
 *  使用场景如下：
 *      1、特意的将数据分配在堆上
 *      2、数据较大时，又不想在转移所有权时进行数据拷贝
 *      3、类型的大小在编译期无法确定，但是我们又需要固定大小的类型时
 *      4、特征对象，用于说明对象实现了一个特征，而不是某个特定的类型
 */
pub fn box_demo() {
    let a = Box::new(5);
    println!("a = {}", a);
    let _b = a;
    // println!("{}", a); // 发生了所有权的转移
}

/** Box最大、最常见的作用：包裹一个动态的类型（Trait）
 * Box 背后是调用 jemalloc 来做内存管理，所以堆上的空间无需我们的手动管理。与此类似，带 GC 的语言中的对象也是借助于 Box 概念来实现的，一切皆对象 = 一切皆 Box， 只不过我们无需自己去 Box 罢了。
 */
trait Draw {
    fn draw(&self);
}

struct Button;
impl Draw for Button {
    fn draw(&self) {
        println!("draw button");
    }
}
struct Select;
impl Draw for Select {
    fn draw(&self) {
        println!("draw select");
    }
}

fn test_1() {
    let value: Vec<Box<dyn Draw>> = vec![Box::new(Button {}), Box::new(Select {})];
    for item in value {
        item.draw();
    }
}
