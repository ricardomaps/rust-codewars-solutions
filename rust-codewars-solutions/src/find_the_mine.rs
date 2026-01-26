pub fn mine_location(field: &[Vec<u8>]) -> (usize, usize) {
    field
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find_map(|(j, cell)| (*cell == 1).then_some((i, j)))
        })
        .unwrap()
}
