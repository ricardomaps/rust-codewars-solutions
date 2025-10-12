use std::cmp::Ordering;
pub fn good_vs_evil(good: &str, evil: &str) -> String {
    let good_worth = vec![1, 2, 3, 3, 4, 10];
    let evil_worth = vec![1, 2, 2, 2, 3, 5, 10];

    let good_score: u32 = good.split(" ")
        .map(|count| count.parse::<u32>().unwrap())
        .zip(good_worth)
        .fold(0, |acc, (score, mult)| score * mult + acc);

    let evil_score: u32 = evil.split(" ")
        .map(|count| count.parse::<u32>().unwrap())
        .zip(evil_worth)
        .fold(0, |acc, (score, mult)| score * mult + acc);

    match good_score.cmp(&evil_score) {
        Ordering::Greater => "Battle Result: Good triumphs over Evil".to_string(),
        Ordering::Less => "Battle Result: Evil eradicates all trace of Good".to_string(),
        Ordering::Equal => "Battle Result: No victor on this battle field".to_string()
    }
}
