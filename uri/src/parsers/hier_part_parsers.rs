use crate::parsers::authority_parsers::authority;
use crate::parsers::path_parsers::{path_abempty, path_rootless};
use oni_comb_parser_rs::prelude::*;

//  hier-part     = "//" authority path-abempty
//                / path-absolute
//                / path-rootless
//                / path-empty
//
// pub fn hier_part<'a>() -> Parser<'a, char, &'a [char]> {
//     ((tag("//") + authority() + path_abempty()).collect() | path_abempty(true) | path_rootless()).opt()
// }
