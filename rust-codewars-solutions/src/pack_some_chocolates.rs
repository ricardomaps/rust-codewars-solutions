fn make_chocolates(small: u32, big: u32, goal: u32) -> Option<u32> {
    let big_quantity = u32::min(goal / 5, big);
    let parity_diff = if big_quantity % 2 != goal % 2 { 1 } else { 0 };
    let rest = goal - 5 * (big_quantity.saturating_sub(parity_diff));
    (rest % 2 == 0 && 2 * small >= rest).then_some(rest / 2)
}
