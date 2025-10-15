pub fn best_match(a: &[u8], b: &[u8]) -> usize {
    let mut index = 0;
    let mut best = (u8::MAX, u8::MIN);
    for i in 0..a.len() {
        let diff = a[i] - b[i];
        if diff < best.0 || diff == best.0 && b[i] > best.1 {
            best = (diff, b[i]);
            index = i;
        }
    }
    index
}
