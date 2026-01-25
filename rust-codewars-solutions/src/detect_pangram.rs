use itertools::Itertools;

fn is_pangram(s: &str) -> bool {
    s.to_lowercase()
        .chars()
        .filter(|ch| ch.is_alphabetic())
        .unique()
        .count()
        == 26
}
