pub fn meeting(s: &str) -> String {
    let mut names = s
        .to_uppercase()
        .split(";")
        .map(|name| name.split_once(":").unwrap())
        .map(|(first, last)| format!("({last}, {first})"))
        .collect::<Vec<String>>();
    names.sort();
    names.join("")
}
