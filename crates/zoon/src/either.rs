use crate::*;
use std::borrow::Cow;

// ------ IntoEither ------

pub trait IntoEither: Sized {
    fn left_either<R>(self) -> Either<Self, R> {
        Either::Left(self)
    }

    fn right_either<L>(self) -> Either<L, Self> {
        Either::Right(self)
    }
}

impl<T> IntoEither for T {}

// ------ Either ------

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

// -- Element for Either --

impl<L: Element, R: Element> Element for Either<L, R> {
    fn into_raw_element(self) -> RawElement {
        match self {
            Either::Left(element) => element.into_raw_element(),
            Either::Right(element) => element.into_raw_element(),
        }
    }
}

// -- IntoCowStr for Either --

impl<'a, L: IntoCowStr<'a>, R: IntoCowStr<'a>> IntoCowStr<'a> for Either<L, R> {
    fn into_cow_str(self) -> Cow<'a, str> {
        match self {
            Either::Left(into_cow_str) => into_cow_str.into_cow_str(),
            Either::Right(into_cow_str) => into_cow_str.into_cow_str(),
        }
    }

    fn take_into_cow_str(&mut self) -> Cow<'a, str> {
        match self {
            Either::Left(into_cow_str) => into_cow_str.take_into_cow_str(),
            Either::Right(into_cow_str) => into_cow_str.take_into_cow_str(),
        }
    }
}

// -- Color for Either --

impl<'a, L: Color<'a>, R: Color<'a>> Color<'a> for Either<L, R> {}