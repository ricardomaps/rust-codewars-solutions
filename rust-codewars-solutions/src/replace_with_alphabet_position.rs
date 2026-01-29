use itertools::Itertools;

fn alphabet_position(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter(|b| b.is_alphabetic())
        .map(|b| (b as u32 - 96).to_string())
        .join(" ")
}
