#[derive(Debug, PartialEq, Eq)]
enum Cons<T: Clone> {
    Cons(T, Box<Cons<T>>),
    Null,
}

impl<T: Clone> Cons<T> {
    pub fn new(head: T, tail: Self) -> Self {
        Cons::Cons(head, Box::new(tail))
    }

    pub fn to_vec(&self) -> Vec<T> {
        match self {
            &Cons::Null => vec![],
            &Cons::Cons(ref head, ref tail) => {
                let mut head = vec![head.clone()];
                head.extend(tail.to_vec());
                head
            }
        }
    }
}

impl<T: Clone> Cons<T> {
    pub fn from_iter<I>(it: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut it = it.into_iter();
        match it.next() {
            Some(el) => Cons::new(el, Cons::from_iter(it)),
            None => Cons::Null,
        }
    }

    pub fn filter<F>(&self, fun: F) -> Self
    where
        F: Fn(&T) -> bool,
    {
        match self {
            Cons::Cons(head, tail) if fun(head) => Cons::new(head.clone(), tail.filter(fun)),
            Cons::Cons(_, tail) => tail.filter(fun),
            Cons::Null => Cons::Null,
        }
    }

    pub fn map<F, S>(&self, fun: F) -> Cons<S>
    where
        F: Fn(T) -> S,
        S: Clone,
    {
        match self {
            Cons::Cons(head, tail) => Cons::new(fun(head.clone()), tail.map(fun)),
            Cons::Null => Cons::Null,
        }
    }
}
