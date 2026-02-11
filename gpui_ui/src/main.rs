use std::ops::Add;

fn main() {
    let a = A;
    def(&a);

    let b = B;
    def(&b);
}

fn add<T>(x: T, y: T) -> T
where
    T: Add<Output = T>,
{
    x + y
}

trait Article {
    fn publish(&self);
}

struct A;

impl Article for A {
    fn publish(&self) {
        println!("publish A");
    }
}

struct B;

impl Article for B {
    fn publish(&self) {
        println!("publish B");
    }
}

fn abc(t: &str) -> Box<dyn Article> {
    match t {
        "a" => Box::new(A),
        "b" => Box::new(B),
        _ => panic!("invalid article type"),
    }
}

fn def(temp: &dyn Article) {
    temp.publish();
}
