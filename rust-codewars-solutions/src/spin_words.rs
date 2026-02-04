use itertools::Itertools;

fn spin_words(words: &str) -> String {
    words
        .split_whitespace()
        .map(|word| {
            if word.len() >= 5 {
                word.chars().rev().collect::<String>()
            } else {
                word.to_string()
            }
        })
        .join(" ")
}
