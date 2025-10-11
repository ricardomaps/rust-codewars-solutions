#[allow(dead_code)]
pub fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
    let mut res = Vec::new();
    if let Some(smallest) = numbers.iter().min() {
        let index = numbers.iter().position(|n| n == smallest).unwrap();
        res.extend_from_slice(&numbers[..index]);
        res.extend_from_slice(&numbers[index+1..]);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_smallest_test() {
        assert_eq!(remove_smallest(&[1, 2, 3, 4 ,5]), vec![2, 3, 4, 5]);
        assert_eq!(remove_smallest(&[]), vec![]);
        assert_eq!(remove_smallest(&[1, 2, 3, 1, 1]), vec![2, 3, 1, 1]);
    }
}
