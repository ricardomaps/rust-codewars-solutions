pub fn min_umbrellas(weather: &[&str]) -> usize {
    let (umbrellas_at_home, umbrellas_at_work) = weather
        .iter()
        .enumerate()
        .filter_map(|(i, &weather)| match weather {
            "rainy" | "thunderstorms" => i.is_multiple_of(2).then_some(1).or(Some(-1)),
            _ => None,
        })
        .scan(0, |acc: &mut i32, x| {
            *acc += x;
            Some(*acc)
        })
        .fold((0, 0), |(cur_max, cur_min), x| {
            (cur_max.max(x), cur_min.min(x))
        });

    (umbrellas_at_home - umbrellas_at_work) as usize
}
