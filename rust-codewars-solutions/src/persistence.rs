pub fn persistence(mut num: u64) -> u64 {
    let mul_digits = |mut n| {
        let mut res = 1;
        while n > 0 {
            res *= n % 10;
            n /= 10;
        }
        res
    };

    let mut times = 0;
    while num > 10 {
        num = mul_digits(num);
        times += 1;
    }
    times
}
