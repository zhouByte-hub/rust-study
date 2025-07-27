pub mod chapter_1;
pub mod chapter_2;


#[cfg(test)]
mod tests {
    use crate::chapter_1::{practice::fibonacci};


    #[test]
    fn it_works() {
        let result = fibonacci(5);
        assert_eq!(result, 5);
    }

}