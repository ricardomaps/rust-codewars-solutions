pub fn fusc(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        n if n % 2 == 0 => fusc(n / 2),
        n => fusc(n / 2) + fusc(n / 2 + 1),
    }
}
