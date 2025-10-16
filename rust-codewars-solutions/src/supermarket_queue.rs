use std::cmp::Reverse;
use std::collections::BinaryHeap;
pub fn queue_time(customers: &[u32], n: u32) -> u32 {
    let mut total_time = 0;
    let mut heap = BinaryHeap::new();
    (0..n).for_each(|_| heap.push(Reverse(0)));
    for customer in customers.iter() {
        let time = heap.pop().unwrap().0 + customer;
        total_time = total_time.max(time);
        heap.push(Reverse(time));
    }
    total_time
}
