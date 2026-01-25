use itertools::Itertools;

fn first_n_smallest(arr: &[i32], n: usize) -> Vec<i32> {
    arr.iter()
        .enumerate()
        .sorted_by_key(|(_, v)| **v)
        .take(n)
        .sorted()
        .map(|(_, &v)| v)
        .collect()
}
