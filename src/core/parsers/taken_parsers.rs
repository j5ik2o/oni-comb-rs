use std::fmt::Debug;
use crate::core::{Element, ElementParsers};

pub trait TakenParsers: ElementParsers {
    fn take<'a, I>(n: usize) -> Self::P<'a, I, &'a [I]>;

    fn take_while0<'a, I, F>(f: F) -> Self::P<'a, I, &'a [I]>
        where
            F: Fn(&I) -> bool + 'a,
            I: Element + Debug + 'a;

    fn take_while1<'a, I, F>(f: F) -> Self::P<'a, I, &'a [I]>
        where
            F: Fn(&I) -> bool + 'a,
            I: Element + Debug + 'a;

    fn take_while_n_m<'a, I, F>(n: usize, m: usize, f: F) -> Self::P<'a, I, &'a [I]>
        where
            F: Fn(&I) -> bool + 'a,
            I: Element + Debug + 'a;

    fn take_till0<'a, I, F>(f: F) -> Self::P<'a, I, &'a [I]>
        where
            F: Fn(&I) -> bool + 'a,
            I: Element + Debug + 'a;

    fn take_till1<'a, I, F>(f: F) -> Self::P<'a, I, &'a [I]>
        where
            F: Fn(&I) -> bool + 'a,
            I: Element + Debug + 'a;

}