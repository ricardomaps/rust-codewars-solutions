struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.curr;
        self.curr = self.next;
        self.next = curr + self.next;
        Some(self.curr)
    }
}

pub fn all_fibonacci_numbers() -> impl Iterator<Item = u64> {
    Fibonacci { curr: 1, next: 1 }
}
