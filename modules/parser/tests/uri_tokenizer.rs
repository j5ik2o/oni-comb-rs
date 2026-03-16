//! URI tokenizer 統合テスト (MS4 完了条件の実証)
//!
//! RFC 3986 の簡易サブセット:
//! scheme "://" host [":" port] ["/" path] ["?" query]

use oni_comb_parser::error::ParseError;
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;

#[derive(Debug, PartialEq)]
struct Uri<'a> {
    scheme: &'a str,
    host: &'a str,
    port: Option<i64>,
    path: Option<&'a str>,
    query: Option<&'a str>,
}

fn uri_parser<'a>() -> impl Parser<
    oni_comb_parser::str_input::StrInput<'a>,
    Output = Uri<'a>,
    Error = ParseError,
> {
    let scheme = take_while1(|c: char| c.is_ascii_alphanumeric());
    let authority_sep = tag("://");
    let host = take_while1(|c: char| c.is_ascii_alphanumeric() || c == '.' || c == '-');
    let port = char(':').zip_right(integer()).optional();
    let path = char('/').zip_right(take_while0(|c: char| {
        c.is_ascii_alphanumeric() || c == '/' || c == '-' || c == '_' || c == '.'
    }))
    .optional();
    let query = char('?').zip_right(take_while0(|c: char| {
        c.is_ascii_alphanumeric() || c == '&' || c == '=' || c == '-' || c == '_' || c == '.'
    }))
    .optional();

    scheme
        .zip_left(authority_sep)
        .zip(host)
        .zip(port)
        .zip(path)
        .zip(query)
        .map(|((((s, h), p), path), q)| Uri {
            scheme: s,
            host: h,
            port: p,
            path,
            query: q,
        })
}

#[test]
fn parse_simple_uri() {
    let mut parser = uri_parser();
    let mut input = StrInput::new("http://example.com");

    assert_eq!(
        parser.parse_next(&mut input).unwrap(),
        Uri {
            scheme: "http",
            host: "example.com",
            port: None,
            path: None,
            query: None,
        }
    );
}

#[test]
fn parse_uri_with_port() {
    let mut parser = uri_parser();
    let mut input = StrInput::new("https://localhost:8080");

    assert_eq!(
        parser.parse_next(&mut input).unwrap(),
        Uri {
            scheme: "https",
            host: "localhost",
            port: Some(8080),
            path: None,
            query: None,
        }
    );
}

#[test]
fn parse_uri_with_path() {
    let mut parser = uri_parser();
    let mut input = StrInput::new("https://example.com/api/v1/users");

    assert_eq!(
        parser.parse_next(&mut input).unwrap(),
        Uri {
            scheme: "https",
            host: "example.com",
            port: None,
            path: Some("api/v1/users"),
            query: None,
        }
    );
}

#[test]
fn parse_uri_with_query() {
    let mut parser = uri_parser();
    let mut input = StrInput::new("http://example.com/search?q=rust&page=1");

    assert_eq!(
        parser.parse_next(&mut input).unwrap(),
        Uri {
            scheme: "http",
            host: "example.com",
            port: None,
            path: Some("search"),
            query: Some("q=rust&page=1"),
        }
    );
}

#[test]
fn parse_full_uri() {
    let mut parser = uri_parser();
    let mut input = StrInput::new("https://api.example.com:443/v2/data?format=json");

    assert_eq!(
        parser.parse_next(&mut input).unwrap(),
        Uri {
            scheme: "https",
            host: "api.example.com",
            port: Some(443),
            path: Some("v2/data"),
            query: Some("format=json"),
        }
    );
}

#[test]
fn parse_uri_with_remaining() {
    let mut parser = uri_parser();
    let mut input = StrInput::new("http://example.com/path rest");

    let uri = parser.parse_next(&mut input).unwrap();
    assert_eq!(uri.host, "example.com");
    assert_eq!(uri.path, Some("path"));
    assert_eq!(input.remaining(), " rest");
}

#[test]
fn parse_ftp_scheme() {
    let mut parser = uri_parser();
    let mut input = StrInput::new("ftp://files.example.com/pub");

    assert_eq!(
        parser.parse_next(&mut input).unwrap(),
        Uri {
            scheme: "ftp",
            host: "files.example.com",
            port: None,
            path: Some("pub"),
            query: None,
        }
    );
}
