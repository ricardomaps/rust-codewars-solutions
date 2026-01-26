fn beggars(values: &[u32], n: usize) -> Vec<u32> {
    if n == 0 {
        return vec![];
    }
    let mut res = vec![0; n];
    for (i, v) in values.iter().enumerate() {
        res[i % n] += v;
    }
    res
}
