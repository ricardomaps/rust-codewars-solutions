pub fn to_weird_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            word.char_indices()
                .map(|(index, character)| {
                    if index % 2 == 0 {
                        character.to_ascii_uppercase()
                    } else {
                        character.to_ascii_lowercase()
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join(" ")
}
