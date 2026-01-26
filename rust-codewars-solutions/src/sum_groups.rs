use itertools::Itertools;

fn sum_groups(arr: &[u32]) -> usize {
    let mut arr = arr.iter().map(|n| *n).collect_vec();
    loop {
        let prev_len = arr.len();
        arr = arr
            .iter()
            .group_by(|n| **n % 2)
            .into_iter()
            .map(|(_, g)| g.sum())
            .collect::<Vec<u32>>();
        if prev_len == arr.len() {
            break arr.len();
        }
    }
}
