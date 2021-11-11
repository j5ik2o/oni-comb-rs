//  scheme        = ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )

use crate::models::scheme::Scheme;
use oni_comb_parser_rs::prelude::*;
use std::iter::FromIterator;

pub fn scheme<'a>() -> Parser<'a, char, Scheme> {
  ((elm_alpha() + (elm_alpha() | elm_digit() | elm_of("+-.")).of_many0()).collect())
    .map(String::from_iter)
    .map(Scheme::new)
}
