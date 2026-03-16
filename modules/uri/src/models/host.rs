use core::fmt;
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Host<'a> {
  RegName(&'a str),
  Ipv4(Ipv4Addr),
  Ipv6(Ipv6Addr),
  IpvFuture(&'a str),
}

impl fmt::Display for Host<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Host::RegName(name) => write!(f, "{}", name),
      Host::Ipv4(addr) => write!(f, "{}", addr),
      Host::Ipv6(addr) => write!(f, "[{}]", addr),
      Host::IpvFuture(s) => write!(f, "[{}]", s),
    }
  }
}
