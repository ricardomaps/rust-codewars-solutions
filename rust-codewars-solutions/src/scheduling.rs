use itertools::Itertools;

fn sjf(jobs: &[usize], index: usize) -> usize {
    jobs.iter()
        .enumerate()
        .sorted_by_key(|(_, v)| *v)
        .take_while(|(i, _)| *i != index)
        .map(|(_, v)| v)
        .sum::<usize>()
        + jobs[index]
}
