use std::collections::HashMap;
pub fn duplicate_encode(word:&str)->String {
    let mut frequencies: HashMap<char, i32> = HashMap::new();
    let word = word.to_lowercase();
    for c in word.chars() {
        frequencies
            .entry(c)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }

    let mut res = String::new();
    for c in word.chars() {
        res.push(if frequencies[&c] > 1 { ')' } else { '(' });
    }
    res
}
