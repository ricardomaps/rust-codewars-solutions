use itertools::Itertools;

fn sort_numbers(arr: &Vec<i32>) -> Vec<i32> {
    arr.iter().sorted().map(|n| *n).collect_vec()
}
