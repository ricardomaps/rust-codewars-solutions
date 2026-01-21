macro_rules! add {
    ($($number:expr),*) => {{
        let mut res = 0;
        $(
            res += $number;
        )*
        res
    }};
}
