pub fn fizz_buzz(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "fizzbuzz".to_string(),
        (0, _) => "fizz".to_string(),
        (_, 0) => "buzz".to_string(),
        _ => n.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_number() {
        assert_eq!(fizz_buzz(1), "1");
        assert_eq!(fizz_buzz(2), "2");
    }

    #[test]
    fn returns_fizz() {
        assert_eq!(fizz_buzz(3), "fizz");
        assert_eq!(fizz_buzz(6), "fizz");
    }

    #[test]
    fn returns_buzz() {
        assert_eq!(fizz_buzz(5), "buzz");
        assert_eq!(fizz_buzz(10), "buzz");
    }

    #[test]
    fn returns_fizzbuzz() {
        assert_eq!(fizz_buzz(15), "fizzbuzz");
        assert_eq!(fizz_buzz(30), "fizzbuzz");
    }
}

fn main() {
    let numbers = vec![1, 3, 5, 15, 16, 18, 20, 30];

    for number in numbers {
        println!("{}: {}", number, fizz_buzz(number));
    }
}
