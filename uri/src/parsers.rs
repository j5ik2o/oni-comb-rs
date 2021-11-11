mod authority_parsers;
mod basic_parsers;
mod host_parsers;
mod ip_v4_address_parsers;
mod ip_v6_address_parsers;
mod path_parsers;
mod port_parsers;
mod scheme_parsers;
mod user_info_parsers;

use crate::parsers::basic_parsers::{pchar, pct_encoded, sub_delims, unreserved};

use oni_comb_parser_rs::prelude::*;
use std::iter::FromIterator;

//  query         = *( pchar / "/" / "?" )
fn query<'a>() -> Parser<'a, char, &'a [char]> {
  (pchar() | elm_of("/?").collect()).of_many0().collect().name("query")
}

// fragment      = *( pchar / "/" / "?" )
fn fragment<'a>() -> Parser<'a, char, &'a [char]> {
  (pchar() | elm_of("/?").collect()).of_many0().collect().name("fragment")
}
