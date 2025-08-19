/**
 * num_cpus = "1.17.0"
 *
 * 获取机器上的 CPU 数量。
 */
#[cfg(test)]
mod cpu_test {

    #[test]
    fn test() {
        let count = num_cpus::get(); // Returns the number of available CPUs of the current system.
        let physical = num_cpus::get_physical(); // Returns the number of physical cores of the current system.

        // 可以通过std::thread获取可用的线程数
        let thread_count = std::thread::available_parallelism().unwrap().get();
        println!("{}-{}-{}", count, physical, thread_count);
    }
}
