pub fn bingo<S: AsRef<str>>(ticket: &[(S, u8)], win: usize) -> &'static str {
    let mini_wins = ticket
        .iter()
        .filter(|(s, code)| s.as_ref().chars().any(|c| c as u8 == *code))
        .count();
    if mini_wins >= win {
        "Winner!"
    } else {
        "Loser!"
    }
}
