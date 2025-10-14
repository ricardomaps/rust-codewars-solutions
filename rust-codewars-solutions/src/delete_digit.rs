pub fn delete_digit(n: u32) -> u32 {
    let mut n: Vec<u8> = n.to_string().chars().map(|c| c as u8).collect();
    for (i, digit) in n.iter().enumerate() {
        let next_digit = n.get(i + 1).or(Some(&u8::MAX)).unwrap();
        if next_digit > digit {
            n.remove(i);
            return String::from_utf8(n).unwrap().parse().unwrap();
        }
    }
    unreachable!();
}
