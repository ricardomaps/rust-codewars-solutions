use itertools::Itertools;

pub fn first_non_repeating(s: &str) -> Option<char> {
    let dup = s.to_lowercase().chars().counts();
    s.chars()
        .find(|c| *dup.get(&c.to_ascii_lowercase()).unwrap() == 1)
}
