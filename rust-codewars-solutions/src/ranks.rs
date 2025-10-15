use std::collections::HashMap;
pub fn ranks(a: &[i32]) -> Vec<usize> {
    let mut v: Vec<i32> = a.into();
    v.sort();
    let mut mapping = HashMap::new();
    for (i, e) in v.iter().rev().enumerate() {
        if !mapping.contains_key(e) {
            mapping.insert(e, i + 1);
        }
    }
    a.iter().map(|n| mapping[n]).collect()
}
