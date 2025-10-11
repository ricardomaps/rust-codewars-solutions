#[allow(dead_code)]
pub fn validate_pin(pin: &str) -> bool {
    (pin.len() == 4 || pin.len() == 6) && pin.chars().all(|c| c.is_digit(10))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_pin_test() {
        assert_eq!(validate_pin("a123"), false);
        assert_eq!(validate_pin("1234"), true);
        assert_eq!(validate_pin("123"), false);
    }
}
