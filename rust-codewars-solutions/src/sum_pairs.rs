use std::collections::HashSet;
pub fn sum_pairs(ints: &[i32], s: i32) -> Option<(i32, i32)> {
    let mut seen = HashSet::new();
    for n in ints.iter() {
        if seen.contains(&(s - n)) {
            return Some((s - n, *n));
        }
        seen.insert(n);
    }
    None
}
