use oni_comb_parser_rs::prelude::*;

//  pct-encoded   = "%" HEXDIG HEXDIG
pub(crate) fn pct_encoded<'a>() -> Parser<'a, char, &'a [char]> {
  (elm('%') + elm_hex_digit() + elm_hex_digit()).collect()
}

//  unreserved    = ALPHA / DIGIT / "-" / "." / "_" / "~"
pub(crate) fn unreserved<'a>() -> Parser<'a, char, &'a [char]> {
  (elm_alpha() | elm_digit() | elm_of("-._~"))
    .collect()
    .name("unreserved")
}

//  reserved      = gen-delims / sub-delims
pub(crate) fn reserved<'a>() -> Parser<'a, char, &'a [char]> {
  (gen_delims() | sub_delims()).name("reserved")
}

// gen-delims    = ":" / "/" / "?" / "#" / "[" / "]" / "@"
pub(crate) fn gen_delims<'a>() -> Parser<'a, char, &'a [char]> {
  elm_of(":/?#[]@").name("gen_delims").collect()
}

// sub-delims    = "!" / "$" / "&" / "'" / "(" / ")" / "*" / "+" / "," / ";" / "="
pub(crate) fn sub_delims<'a>() -> Parser<'a, char, &'a [char]> {
  elm_of("!$&'()*+,;=").name("sub_delims").collect()
}

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;
    use super::*;

    #[test]
    fn test_pct_encoded() {
        let input = "%1A".chars().collect::<Vec<_>>();
        let result = (pct_encoded().of_many1() - end())
            .collect()
            .map(String::from_iter)
            .parse(&input)
            .to_result();
        assert_eq!(result.unwrap(), "%1A".to_string());
    }

    #[test]
    fn test_unreserved() {
        let input = "1a-._~".chars().collect::<Vec<_>>();
        let result = (unreserved().of_many1() - end())
            .collect()
            .map(String::from_iter)
            .parse(&input)
            .to_result();
        assert_eq!(result.unwrap(), "1a-._~".to_string());
    }

    #[test]
    fn test_reserved() {
        let input = "!$&'()*+,;=:/?#[]@".chars().collect::<Vec<_>>();
        let result = (reserved().of_many1() - end())
            .collect()
            .map(String::from_iter)
            .parse(&input)
            .to_result();
        assert_eq!(result.unwrap(), "!$&'()*+,;=:/?#[]@".to_string());
    }

    #[test]
    fn test_gen_delims() {
        let input = ":/?#[]@".chars().collect::<Vec<_>>();
        let result = (gen_delims().of_many1() - end())
            .collect()
            .map(String::from_iter)
            .parse(&input)
            .to_result();
        assert_eq!(result.unwrap(), ":/?#[]@".to_string());
    }

    #[test]
    fn test_sub_delims() {
        let input = "!$&'()*+,;=".chars().collect::<Vec<_>>();
        let result = (sub_delims().of_many1() - end())
            .collect()
            .map(String::from_iter)
            .parse(&input)
            .to_result();
        assert_eq!(result.unwrap(), "!$&'()*+,;=".to_string());
    }
}