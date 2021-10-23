use std::fmt::Debug;
use crate::core::Parser;
use crate::extension::{RepeatCombinator, RepeatCombinators};
use crate::internal::ParsersImpl;
use crate::utils::RangeArgument;

impl<'a, I, A> RepeatCombinator<'a> for Parser<'a, I, A> {
    fn repeat<R>(self, range: R) -> Self::P<'a, Self::Input, Vec<Self::Output>>
        where
            R: RangeArgument<usize> + Debug + 'a,
            Self::Output: Debug + 'a,
            Self: Sized, {
        ParsersImpl::repeat(self, range)
    }

    fn of_many0(self) -> Self::P<'a, Self::Input, Vec<Self::Output>>
        where
            Self::Output: Debug + 'a, {
        ParsersImpl::many0(self)
    }

    fn of_many1(self) -> Self::P<'a, Self::Input, Vec<Self::Output>>
        where
            Self::Output: Debug + 'a, {
        ParsersImpl::many1(self)
    }

    fn of_many_n_m(self, n: usize, m: usize) -> Self::P<'a, Self::Input, Vec<Self::Output>>
        where
            Self::Output: Debug + 'a, {
        ParsersImpl::many_n_m(self, n, m)
    }

    fn of_count(self, n: usize) -> Self::P<'a, Self::Input, Vec<Self::Output>>
        where
            Self::Output: Debug + 'a, {
        ParsersImpl::count(self, n)
    }

    fn of_rep_sep<B, R>(
        self,
        range: R,
        separator: Option<Self::P<'a, Self::Input, B>>,
    ) -> Self::P<'a, Self::Input, Vec<Self::Output>>
        where
            R: RangeArgument<usize> + Debug + 'a,
            Self::Output: Debug + 'a,
            B: Debug + 'a, {
        ParsersImpl::repeat_sep(self, range, separator)
    }

    fn of_many0_sep<B>(self, separator: Self::P<'a, Self::Input, B>) -> Self::P<'a, Self::Input, Vec<Self::Output>>
        where
            Self::Output: Debug + 'a,
            B: Debug + 'a, {
        ParsersImpl::many0_sep(self, separator)
    }

    fn of_many1_sep<B>(self, separator: Self::P<'a, Self::Input, B>) -> Self::P<'a, Self::Input, Vec<Self::Output>>
        where
            Self::Output: Debug + 'a,
            B: Debug + 'a, {
        ParsersImpl::many1_sep(self, separator)
    }

    fn of_many_n_m_sep<B>(
        self,
        n: usize,
        m: usize,
        separator: Self::P<'a, Self::Input, B>,
    ) -> Self::P<'a, Self::Input, Vec<Self::Output>>
        where
            Self::Output: Debug + 'a,
            B: Debug + 'a, {
        ParsersImpl::many_n_m_sep(self, n, m, separator)
    }

    fn of_count_sep<B>(
        self,
        n: usize,
        separator: Self::P<'a, Self::Input, B>,
    ) -> Self::P<'a, Self::Input, Vec<Self::Output>>
        where
            Self::Output: Debug + 'a,
            B: Debug + 'a, {
        ParsersImpl::count_sep(self, n, separator)
    }
}
