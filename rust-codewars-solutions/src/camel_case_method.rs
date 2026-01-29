use itertools::Itertools;

fn camel_case(str: &str) -> String {
    str.split_whitespace().map(|word| capitalize(word)).join("")
}

fn capitalize(word: impl AsRef<str>) -> String {
    let mut c = word.as_ref().chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
