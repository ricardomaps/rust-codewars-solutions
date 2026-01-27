#![allow(dead_code)]

use std::iter::once;

/// so, when are two type, `a` and `b`, considered equal?
/// a definition might be, it is possible to go from `a` to `b`,
/// and from `b` to `a`.
/// Going a roundway trip should leave you the same value.
/// Unfortunately it is virtually impossible to test this in Haskell,
/// neither in Rust.
/// This is called ISO.
pub enum Void {}

impl PartialEq for Void {
    fn eq(&self, _: &Void) -> bool {
        true
    }
}

pub fn absurd(_: Void) -> ! {
    panic!("You must be kidding! Where did you find that void instance?");
}

pub type Func<A, B> = Box<dyn Fn(A) -> B>;
pub type RetFunc<A, B> = Box<dyn FnOnce(A) -> B>;
pub type ISO<A, B> = (Func<A, B>, Func<B, A>);

pub fn iso<A: 'static, B: 'static, F1, F2>(a: F1, b: F2) -> ISO<A, B>
where
    F1: 'static + Fn(A) -> B,
    F2: 'static + Fn(B) -> A,
{
    (Box::new(a), Box::new(b))
}

///  *** MENTION ***  ///
///  paste your code  ///
/// from isomorphism  ///
///  *** MENTION ***  ///

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

/// Sometimes, we can treat a Type as a Number:
/// if a Type t has n distinct value, it's Number is n.
/// This is formally called cardinality.
/// See https://en.wikipedia.org/wiki/Cardinality
///
/// Void has cardinality of 0 (we will abbreviate it Void is 0).
/// () is 1.
/// Bool is 2.
/// Maybe a is 1 + a.
/// We will be using peano arithmetic so we will write it as S a.
/// https://en.wikipedia.org/wiki/Peano_axioms
/// Either a b is a + b.
/// (a, b) is a * b.
/// a => b is b ^ a. Try counting (() => Bool) and (Bool => ())
///
/// Algebraic data type got the name because
/// it satisfies a lot of algebraic rules under isomorphism    

/// a = b => c = d => a * c = b * d
pub fn iso_prod<A: 'static, B: 'static, C: 'static, D: 'static>(
    ab: ISO<A, B>,
    cd: ISO<C, D>,
) -> ISO<(A, C), (B, D)> {
    iso_tuple(ab, cd)
}

/// a = b => c = d => a + c = b + d
pub fn iso_plus<A: 'static, B: 'static, C: 'static, D: 'static>(
    ab: ISO<A, B>,
    cd: ISO<C, D>,
) -> ISO<Result<A, C>, Result<B, D>> {
    iso_result(ab, cd)
}

/// a = b => S a = S b
pub fn iso_s<A: 'static, B: 'static>(i: ISO<A, B>) -> ISO<Option<A>, Option<B>> {
    iso_option(i)
}

/// a = b => c = d => c ^ a = d ^ b
pub fn iso_pow<A: 'static, B: 'static, C: 'static, D: 'static>(
    ab: ISO<A, B>,
    cd: ISO<C, D>,
) -> IsoF<A, B, C, D> {
    iso_func(ab, cd)
}

/// a + b = b + a
pub fn plus_comm<A: 'static, B: 'static>() -> ISO<Result<A, B>, Result<B, A>> {
    fn swap<A, B>(r: Result<A, B>) -> Result<B, A> {
        match r {
            Ok(x) => Err(x),
            Err(y) => Ok(y),
        }
    }
    iso(swap, swap)
}

/// a + b + c = a + (b + c)
pub fn plus_assoc<A: 'static, B: 'static, C: 'static>()
-> ISO<Result<Result<A, B>, C>, Result<A, Result<B, C>>> {
    iso(
        |r: Result<Result<A, B>, C>| match r {
            Ok(Ok(a)) => Ok(a),
            Ok(Err(b)) => Err(Ok(b)),
            Err(c) => Err(Err(c)),
        },
        |r: Result<A, Result<B, C>>| match r {
            Ok(a) => Ok(Ok(a)),
            Err(Ok(b)) => Ok(Err(b)),
            Err(Err(c)) => Err(c),
        },
    )
}

/// a * b = b * a
pub fn mult_comm<A: 'static, B: 'static>() -> ISO<(A, B), (B, A)> {
    fn swap<A, B>((x, y): (A, B)) -> (B, A) {
        (y, x)
    }
    iso(swap, swap)
}

/// a * b * c = a * (b * c)
pub fn mult_assoc<A: 'static, B: 'static, C: 'static>() -> ISO<((A, B), C), (A, (B, C))> {
    iso(
        |((x, y), z): ((A, B), C)| (x, (y, z)),
        |(x, (y, z)): (A, (B, C))| ((x, y), z),
    )
}

/// a * (b + c) = a * b + a * c
pub fn dist<A: 'static, B: 'static, C: 'static>() -> ISO<(A, Result<B, C>), Result<(A, B), (A, C)>>
{
    iso(
        |(a, r): (A, Result<B, C>)| match r {
            Ok(b) => Ok((a, b)),
            Err(c) => Err((a, c)),
        },
        |r: Result<(A, B), (A, C)>| match r {
            Ok((a, b)) => (a, Ok(b)),
            Err((a, c)) => (a, Err(c)),
        },
    )
}

pub type IsoCL<A, B, C> = RetFunc<Func<A, Func<B, C>>, RetFunc<(A, B), C>>;
pub type IsoCR<A, B, C> = RetFunc<Func<(A, B), C>, RetFunc<A, RetFunc<B, C>>>;
pub type IsoC<A, B, C> = (IsoCL<A, B, C>, IsoCR<A, B, C>);

/// translator note:
/// FnBox is not yet supported, we can only return an uncallable
/// Box<FnOnce> (RetFunc). You should return the function with
/// correct type, which will be checked by the tests.
/// later you'll have to implement three more functions that related
/// to `RetFunc`. enjoy!

/// (c ^ b) ^ a = c ^ (a * b)
pub fn curry_iso<A: 'static, B: 'static, C: 'static>() -> IsoC<A, B, C> {
    (
        Box::new(|f: Func<A, Func<B, C>>| Box::new(move |(a, b): (A, B)| (f(a))(b))),
        Box::new(|f: Func<(A, B), C>| Box::new(|a: A| Box::new(move |b: B| f((a, b))))),
    )
}

/// 1 = S O (we are using peano arithmetic)
/// https://en.wikipedia.org/wiki/Peano_axioms  
pub fn one() -> ISO<(), Option<Void>> {
    iso(
        |()| None,
        |x: Option<Void>| match x {
            Some(void) => absurd(void),
            None => (),
        },
    )
}

/// 2 = S (S O)
pub fn two() -> ISO<bool, Option<Option<Void>>> {
    iso(
        |b: bool| if b { Some(None) } else { None },
        |x: Option<Option<Void>>| match x {
            Some(_) => true,
            None => false,
        },
    )
}

/// 0 + b = b
pub fn plus_o<B: 'static>() -> ISO<Result<Void, B>, B> {
    iso(
        |r: Result<Void, B>| match r {
            Ok(void) => absurd(void),
            Err(b) => b,
        },
        |b: B| Err(b),
    )
}

/// S a + b = S (a + b)
pub fn plus_s<A: 'static, B: 'static>() -> ISO<Result<Option<A>, B>, Option<Result<A, B>>> {
    iso(|r: Result<Option<A>, B>| r.transpose(), |r| r.transpose())
}

/// 1 + b = S b
pub fn plus_so<B: 'static>() -> ISO<Result<(), B>, Option<B>> {
    trans(iso_plus(one(), refl()), trans(plus_s(), iso_s(plus_o())))
}

/// 0 * a = 0
pub fn mult_o<A: 'static>() -> ISO<(Void, A), Void> {
    iso(|t: (Void, A)| absurd(t.0), |v: Void| absurd(v))
}

/// S a * b = b + a * b
pub fn mult_s<A: 'static, B: 'static>() -> ISO<(Option<A>, B), Result<B, (A, B)>> {
    iso(
        |t: (Option<A>, B)| match t.0 {
            Some(a) => Err((a, t.1)),
            None => Ok(t.1),
        },
        |r: Result<B, (A, B)>| match r {
            Ok(b) => (None, b),
            Err((a, b)) => (Some(a), b),
        },
    )
}

/// S a * b = b + a * b
pub fn mult_so<B: 'static>() -> ISO<((), B), B> {
    trans(
        iso_prod(one(), refl()),
        trans(
            mult_s(),
            trans(iso_plus(refl(), mult_o()), trans(plus_comm(), plus_o())),
        ),
    )
}

/// Here we go, the last three functions related to
/// RetFunc. They're easy!

pub type IsoPL<A> = RetFunc<Func<Void, A>, ()>;
pub type IsoPR<A> = RetFunc<(), RetFunc<Void, A>>;
pub type IsoP<A> = (IsoPL<A>, IsoPR<A>);

/// a ^ 0 = 1
pub fn pow_o<A: 'static>() -> IsoP<A> {
    (
        Box::new(|_: Func<Void, A>| ()),
        Box::new(|()| Box::new(|void| absurd(void))),
    )
}

pub type IsoPsL<A, B> = RetFunc<Func<Option<B>, A>, (A, RetFunc<B, A>)>;
pub type IsoPsR<A, B> = RetFunc<(A, Func<B, A>), RetFunc<Option<B>, A>>;
pub type IsoPs<A, B> = (IsoPsL<A, B>, IsoPsR<A, B>);

/// a ^ (S b) = a * (a ^ b)
pub fn pow_s<A: 'static, B: 'static>() -> IsoPs<A, B> {
    (
        Box::new(|f: Func<Option<B>, A>| (f(None), Box::new(move |b: B| f(Some(b))))),
        Box::new(|(a, f): (A, Func<B, A>)| Box::new(|b: Option<B>| b.map_or(a, f))),
    )
}

pub type IsoPsoL<A> = RetFunc<Func<(), A>, A>;
pub type IsoPsoR<A> = RetFunc<A, RetFunc<(), A>>;
pub type IsoPso<A> = (IsoPsoL<A>, IsoPsoR<A>);

/// a ^ 1 = a
/// In Haskell/Java/Dart, you can go the hard way
/// (like mult_so, plus_so) to prove that you really get what is
/// going on.
/// Unfortunately, in Rust, you can only go the trivial way.
/// Because Rust doesn't support FnBox ATM.
pub fn pow_so<A: 'static>() -> IsoPso<A> {
    (
        Box::new(|f: Func<(), A>| f(())),
        Box::new(|a: A| Box::new(|()| a)),
    )
}
