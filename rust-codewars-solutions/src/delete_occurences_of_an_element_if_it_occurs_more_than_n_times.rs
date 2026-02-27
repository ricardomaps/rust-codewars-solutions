use std::collections::HashMap;

fn delete_nth(lst: &[u8], n: usize) -> Vec<u8> {
    let mut counts = HashMap::new();
    lst.iter()
        .copied()
        .filter(|&x| *counts.entry(x).and_modify(|e| *e += 1).or_insert(1) <= n)
        .collect()
}
