fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
    let mut are_in: Vec<String> = arr_a
        .iter()
        .filter(|&word| arr_b.iter().any(|word2| word2.contains(word)))
        .map(|&word| word.to_owned())
        .collect();
    are_in.sort();
    are_in.dedup();
    are_in
}
