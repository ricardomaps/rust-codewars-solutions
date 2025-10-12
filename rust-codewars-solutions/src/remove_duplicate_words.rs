use std::collections::HashSet;
pub fn remove_duplicate_words(s: &str) -> String {
    let mut seen = HashSet::new();
    s.split(" ").filter(|word| seen.insert(word.clone())).collect::<Vec<&str>>().join(" ")

}
