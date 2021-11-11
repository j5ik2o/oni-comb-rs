//  scheme        = ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )

use oni_comb_parser_rs::prelude::*;

pub fn scheme<'a>() -> Parser<'a, char, &'a [char]> {
  (elm_alpha() + (elm_alpha() | elm_digit() | elm_of("+-.")).of_many0()).collect()
}
