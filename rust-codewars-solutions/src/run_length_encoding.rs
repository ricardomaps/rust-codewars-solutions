use itertools::Itertools;

fn run_length_encoding(s: &str) -> Vec<(usize, char)> {
    s.chars().dedup_with_count().collect()
}
