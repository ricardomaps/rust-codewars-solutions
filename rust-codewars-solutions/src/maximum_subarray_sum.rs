fn max_sequence(seq: &[i32]) -> i32 {
    let mut res = 0;
    let mut cur = 0;
    for n in seq {
        cur += n;
        if cur < *n {
            cur = *n;
        }
        res = if cur > res { cur } else { res };
    }
    res
}
