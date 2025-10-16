macro_rules! zero {
    () => {
        0
    };
    ($x: expr) => {
        ($x)(0)
    };
}

macro_rules! one {
    () => {
        1
    };
    ($x: expr) => {
        ($x)(1)
    };
}

macro_rules! two {
    () => {
        2
    };
    ($x: expr) => {
        ($x)(2)
    };
}

macro_rules! three {
    () => {
        3
    };
    ($x: expr) => {
        ($x)(3)
    };
}

macro_rules! four {
    () => {
        4
    };
    ($x: expr) => {
        ($x)(4)
    };
}

macro_rules! five {
    () => {
        5
    };
    ($x: expr) => {
        ($x)(5)
    };
}

macro_rules! six {
    () => {
        6
    };
    ($x: expr) => {
        ($x)(6)
    };
}

macro_rules! seven {
    () => {
        7
    };
    ($x: expr) => {
        ($x)(7)
    };
}

macro_rules! eight {
    () => {
        8
    };
    ($x: expr) => {
        ($x)(8)
    };
}

macro_rules! nine {
    () => {
        9
    };
    ($x: expr) => {
        ($x)(9)
    };
}

pub const fn plus(y: i32) -> impl Fn(i32) -> i32 {
    move |x| x + y
}

pub const fn minus(y: i32) -> impl Fn(i32) -> i32 {
    move |x| x - y
}

pub const fn times(y: i32) -> impl Fn(i32) -> i32 {
    move |x| x * y
}

pub const fn divided_by(y: i32) -> impl Fn(i32) -> i32 {
    move |x| x / y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculating_with_functions() {
        assert_eq!(seven!(times(five!())), 35);
        assert_eq!(eight!(divided_by(three!())), 2);
        assert_eq!(four!(plus(nine!())), 13);
        assert_eq!(nine!(minus(six!())), 3);
    }
}
