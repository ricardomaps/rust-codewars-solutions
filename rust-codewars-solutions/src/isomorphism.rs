#![allow(dead_code)]

use std::iter::once;

/// so, when are two type, `a` and `b`, considered equal?
/// a definition might be, it is possible to go from `a` to `b`,
/// and from `b` to `a`.
/// Going a roundway trip should leave you the same value.
/// Unfortunately it is virtually impossible to test this in Haskell,
/// neither in Rust.
/// This is called Isomorphism.
pub enum Void {}

impl PartialEq for Void {
    fn eq(&self, _: &Void) -> bool {
        true
    }
}

pub fn absurd(_: Void) -> ! {
    panic!("You must be kidding! Where did you find that void instance?");
}

pub type ISO<A: 'static, B: 'static> = (Box<dyn Fn(A) -> B>, Box<dyn Fn(B) -> A>);

pub fn iso<A: 'static, B: 'static, F1, F2>(a: F1, b: F2) -> ISO<A, B>
where
    F1: 'static + Fn(A) -> B,
    F2: 'static + Fn(B) -> A,
{
    (Box::new(a), Box::new(b))
}

/// given ISO a b, we can go from a to b
pub fn sub_st_l<A, B>(iso: ISO<A, B>) -> Box<dyn Fn(A) -> B> {
    iso.0
}

/// and vise versa
pub fn sub_st_r<A, B>(iso: ISO<A, B>) -> Box<dyn Fn(B) -> A> {
    iso.1
}

/// There can be more than one ISO a b
pub fn iso_bool() -> ISO<bool, bool> {
    refl()
}

pub fn iso_bool_not() -> ISO<bool, bool> {
    let not = |b: bool| !b;
    iso(not, not)
}

/// isomorphism is reflexive
pub fn refl<A: 'static>() -> ISO<A, A> {
    let id = |x: A| x;
    iso(id, id)
}

/// isomorphism is symmetric
pub fn symm<A: 'static, B: 'static>(i: ISO<A, B>) -> ISO<B, A> {
    (i.1, i.0)
}

/// isomorphism is transitive
pub fn trans<A: 'static, B: 'static, C: 'static>(ab: ISO<A, B>, bc: ISO<B, C>) -> ISO<A, C> {
    let ac = move |a: A| (bc.0)((ab.0)(a));
    let ca = move |c: C| (ab.1)((bc.1)(c));
    iso(ac, ca)
}

/// we can combine isomorphism
pub fn iso_tuple<A: 'static, B: 'static, C: 'static, D: 'static>(
    ab: ISO<A, B>,
    cd: ISO<C, D>,
) -> ISO<(A, C), (B, D)> {
    let f = move |x: (A, C)| ((ab.0)(x.0), (cd.0)(x.1));
    let g = move |y: (B, D)| ((ab.1)(y.0), (cd.1)(y.1));
    iso(f, g)
}

pub fn iso_vec<A: 'static, B: 'static>(i: ISO<A, B>) -> ISO<Vec<A>, Vec<B>> {
    let f = move |x: Vec<A>| x.into_iter().map(&i.0).collect();
    let g = move |y: Vec<B>| y.into_iter().map(&i.1).collect();
    iso(f, g)
}

pub fn iso_option<A: 'static, B: 'static>(i: ISO<A, B>) -> ISO<Option<A>, Option<B>> {
    let f = move |x: Option<A>| x.map(&i.0);
    let g = move |y: Option<B>| y.map(&i.1);
    iso(f, g)
}

pub fn iso_result<A: 'static, B: 'static, C: 'static, D: 'static>(
    ab: ISO<A, B>,
    cd: ISO<C, D>,
) -> ISO<Result<A, C>, Result<B, D>> {
    let f = move |x: Result<A, C>| x.map(&ab.0).map_err(&cd.0);
    let g = move |y: Result<B, D>| y.map(&ab.1).map_err(&cd.1);
    iso(f, g)
}

/// Going another way is hard (and is generally impossible)
/// Remember, for all valid ISO, converting and converting back
/// is the same as the original value.
/// You need this to prove some case are impossible.
pub fn iso_un_option<A: 'static, B: 'static>(i: ISO<Option<A>, Option<B>>) -> ISO<A, B> {
    let f = move |x: A| match (i.0)(Some(x)) {
        Some(r) => r,
        None => (i.0)(None).unwrap(),
    };
    let g = move |y: B| match (i.1)(Some(y)) {
        Some(r) => r,
        None => (i.1)(None).unwrap(),
    };
    iso(f, g)
}

/// inf + 0 = inf + 1
pub fn iso_eu() -> ISO<Result<Vec<()>, ()>, Result<Vec<()>, Void>> {
    let f = move |x: Result<Vec<()>, ()>| match x {
        Ok(v) => Ok(v.into_iter().chain(once(())).collect()),
        Err(_) => Ok(vec![]),
    };
    let g = move |y: Result<Vec<()>, Void>| match y {
        Ok(v) if v.len() == 0 => Err(()),
        Ok(v) => Ok(v.into_iter().skip(1).collect()),
        Err(void) => absurd(void),
    };
    iso(f, g)
}

pub type IsoFL<A, B, C, D> = Box<dyn FnOnce(Box<dyn Fn(A) -> C>) -> Box<dyn FnOnce(B) -> D>>;
pub type IsoFR<A, B, C, D> = Box<dyn FnOnce(Box<dyn Fn(B) -> D>) -> Box<dyn FnOnce(A) -> C>>;
pub type IsoF<A, B, C, D> = (IsoFL<A, B, C, D>, IsoFR<A, B, C, D>);

/// translator note:
/// You should return the function with correct type,
/// which will be checked by the tests.
/// The type annotation is shown above. You need you return something like
/// (Box::new(...), Box::new(...))
pub fn iso_func<A: 'static, B: 'static, C: 'static, D: 'static>(
    ab: ISO<A, B>,
    cd: ISO<C, D>,
) -> IsoF<A, B, C, D> {
    let f: IsoFL<A, B, C, D> =
        Box::new(move |x: Box<dyn Fn(A) -> C>| Box::new(move |b: B| (cd.0)(x((ab.1)(b)))));

    let g: IsoFR<A, B, C, D> =
        Box::new(move |y: Box<dyn Fn(B) -> D>| Box::new(move |a: A| (cd.1)(y((ab.0)(a)))));

    (Box::new(f), Box::new(g))
}

/// And we have isomorphism on isomorphism!
pub fn iso_symm<A: 'static, B: 'static>() -> ISO<ISO<A, B>, ISO<B, A>> {
    (Box::new(symm), Box::new(symm))
}
