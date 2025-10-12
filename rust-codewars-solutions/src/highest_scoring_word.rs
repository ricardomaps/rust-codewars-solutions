pub fn high(input: &str) -> &str {
    input
        .split_whitespace()
        .rev()
        .max_by_key(|w| w.as_bytes().iter().map(|b| *b as u32 - 96).sum::<u32>())
        .unwrap()
}
