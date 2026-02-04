use std::collections::HashSet;
use std::hash::Hash;

fn array_diff<T: PartialEq + Eq + Hash>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let b: HashSet<T> = b.into_iter().collect();
    a.into_iter().filter(|x| !b.contains(&x)).collect()
}
