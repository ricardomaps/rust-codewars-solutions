use std::iter::once;

fn parts_sums(ls: &[u64]) -> Vec<u64> {
    once(0)
        .chain(ls.iter().rev().scan(0, |acc, n| {
            *acc += n;
            Some(*acc)
        }))
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect()
}
