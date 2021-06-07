// iterators4.rs

// I AM DONE

pub fn factorial(num: u64) -> u64 {
// This solution works
// (1..=num).fold(1, | acc, v| acc*v) 
// This solution also works
(1..=num).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
