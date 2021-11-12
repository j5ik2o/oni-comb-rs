use std::fmt::Formatter;
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum HostName {
  RegName(String),
  Ipv4Address(Ipv4Addr),
  IpLiteral(IpLiteral),
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum IpLiteral {
  Ipv6Address(Ipv6Addr),
  IpvFuture(String),
}

impl std::fmt::Display for IpLiteral {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      IpLiteral::Ipv6Address(ip) => write!(f, "[{}]", ip),
      IpLiteral::IpvFuture(future) => write!(f, "[{}]", future),
    }
  }
}

impl Default for HostName {
  fn default() -> Self {
    HostName::RegName(String::default())
  }
}

impl std::fmt::Display for HostName {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      HostName::RegName(reg_name) => write!(f, "{}", reg_name),
      HostName::Ipv4Address(ipv4_addr) => write!(f, "{}", ipv4_addr),
      HostName::IpLiteral(ip_literal) => write!(f, "{}", ip_literal),
    }
  }
}
