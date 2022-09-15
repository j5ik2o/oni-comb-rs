#![warn(dead_code)]
mod expr;
pub mod models;
mod parsers;

#[cfg(test)]
mod tests {
  use crate::models::uri::Uri;
  use http::Uri as HttpUri;

  #[test]
  fn it_works() {
    let text = "http://localhost";
    let http_uri = text.parse::<HttpUri>().unwrap();
    println!("{:?}", http_uri.host());

    let oni_comb_uri = Uri::parse(text).unwrap();
    println!("{:?}", oni_comb_uri.authority().unwrap().host_name());
  }
}
