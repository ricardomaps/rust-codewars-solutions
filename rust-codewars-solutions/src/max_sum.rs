pub fn max_sum(arr: &[i32], ranges: &[(usize, usize)]) -> i32 {
    let prefix_sums: Vec<i32> = arr
        .iter()
        .chain(std::iter::once(&0))
        .scan(0, |acc, n| {
            *acc += n;
            Some(*acc)
        })
        .collect();

    ranges
        .iter()
        .map(|&(first, last)| prefix_sums[last + 1] - prefix_sums[first])
        .max()
        .unwrap()
}
