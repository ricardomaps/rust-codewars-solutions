fn luck_check(ticket: &str) -> Option<bool> {
    if ticket.is_empty() {
        return None;
    }
    let mid = ticket.len() / 2;
    let skip = ticket.len() % 2;
    let digits = ticket
        .chars()
        .map(|ch| ch.to_digit(10))
        .collect::<Option<Vec<_>>>()?;
    let left_sum: u32 = digits.iter().take(mid).sum();
    let right_sum: u32 = digits.iter().skip(mid + skip).sum();
    Some(left_sum == right_sum)
}
