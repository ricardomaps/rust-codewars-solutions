use std::cmp::Ordering;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Order {
    Unordered,
    Increasing,
    NotDecreasing,
    Decreasing,
    NotIncreasing,
    Constant,
}
pub fn sequence_classifier(arr: &[i32]) -> Order {
    if arr.len() <= 1 {
        return Order::Constant;
    }

    let mut order = match arr[0].cmp(&arr[1]) {
        Ordering::Less => Order::Increasing,
        Ordering::Equal => Order::Constant,
        Ordering::Greater => Order::Decreasing,
    };

    let mut prev = arr[1];
    for el in arr.iter().skip(2) {
        order = match (order, el.cmp(&prev)) {
            (Order::Increasing, Ordering::Greater)
            | (Order::NotDecreasing, Ordering::Greater)
            | (Order::NotDecreasing, Ordering::Equal)
            | (Order::Decreasing, Ordering::Less)
            | (Order::NotIncreasing, Ordering::Less)
            | (Order::NotIncreasing, Ordering::Equal)
            | (Order::Constant, Ordering::Equal) => order,
            (Order::Increasing, Ordering::Equal) => Order::NotDecreasing,
            (Order::Decreasing, Ordering::Equal) => Order::NotIncreasing,
            (Order::Constant, Ordering::Less) => Order::NotIncreasing,
            (Order::Constant, Ordering::Greater) => Order::NotDecreasing,
            _ => Order::Unordered,
        };
        if order == Order::Unordered {
            return order;
        }
        prev = *el;
    }

    order
}
