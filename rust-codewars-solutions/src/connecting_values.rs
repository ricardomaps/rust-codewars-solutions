use std::collections::HashSet;

fn connected_values(arr: &[Vec<u8>], val: u8, coord: (usize, usize)) -> Vec<(usize, usize)> {
    if arr[coord.0][coord.1] != val {
        return Vec::new();
    }
    let mut vis = HashSet::from([coord]);
    let mut stack = Vec::from([coord]);
    while let Some((x, y)) = stack.pop() {
        for x in x.saturating_sub(1)..=(x + 1) {
            for y in y.saturating_sub(1)..=(y + 1) {
                if x >= arr.len() || y >= arr[0].len() {
                    continue;
                }
                if arr[x][y] == val && !vis.contains(&(x, y)) {
                    vis.insert((x, y));
                    stack.push((x, y));
                }
            }
        }
    }
    vis.iter().copied().collect()
}
