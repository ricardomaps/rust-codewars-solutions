use itertools::Itertools;

fn separate_liquids(glass: &[Vec<char>]) -> Vec<Vec<char>> {
    let Some(glass_size) = glass.first().map(|glass| glass.len()) else {
        return vec![];
    };
    glass
        .concat()
        .into_iter()
        .sorted_by_key(|ch| match ch {
            'O' => 0,
            'A' => 1,
            'W' => 2,
            'H' => 3,
            _ => panic!("not allowed"),
        })
        .chunks(glass_size)
        .into_iter()
        .map(|c| c.collect_vec())
        .collect_vec()
}
