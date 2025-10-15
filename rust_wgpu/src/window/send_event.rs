use std::fmt::Debug;

pub trait CustomEvent: Debug {
    fn run(&self);
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct TestApp {}

#[allow(dead_code)]
impl TestApp {
    pub fn new() -> Self {
        Self {}
    }

    // 模拟鼠标点击
    fn simulate_click(&self, x: f64, y: f64) {
        println!("simulate_click({}, {})", x, y);
    }
}

impl CustomEvent for TestApp {
    fn run(&self) {
        self.simulate_click(100.0, 100.0);
    }
}
