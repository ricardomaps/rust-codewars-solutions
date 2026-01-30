use itertools::{Itertools, repeat_n};
use std::cmp::Reverse;

fn solve(vec: &[i32]) -> Vec<i32> {
    vec.iter()
        .counts()
        .into_iter()
        .sorted_by_key(|(c, n)| (Reverse(*n), *c))
        .flat_map(|(c, n)| repeat_n(*c, n))
        .collect_vec()
}
