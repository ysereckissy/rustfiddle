pub fn add_one(x: i32) -> i32 {
   x + 1
}
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn it_does_the_job() {
        assert_eq!(3, add_one(2));
    }
}
