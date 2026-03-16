use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Path<'a> {
  Abempty(Vec<&'a str>),
  Absolute(Vec<&'a str>),
  Rootless(Vec<&'a str>),
  NoScheme(Vec<&'a str>),
  Empty,
}

impl<'a> Path<'a> {
  pub fn segments(&self) -> &[&'a str] {
    match self {
      Path::Abempty(s) | Path::Absolute(s) | Path::Rootless(s) | Path::NoScheme(s) => s,
      Path::Empty => &[],
    }
  }

  pub fn is_empty(&self) -> bool {
    matches!(self, Path::Empty)
  }
}

impl fmt::Display for Path<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Path::Abempty(segs) => {
        for seg in segs {
          write!(f, "/{}", seg)?;
        }
        Ok(())
      }
      Path::Absolute(segs) => {
        write!(f, "/")?;
        for (i, seg) in segs.iter().enumerate() {
          if i > 0 {
            write!(f, "/")?;
          }
          write!(f, "{}", seg)?;
        }
        Ok(())
      }
      Path::Rootless(segs) | Path::NoScheme(segs) => {
        for (i, seg) in segs.iter().enumerate() {
          if i > 0 {
            write!(f, "/")?;
          }
          write!(f, "{}", seg)?;
        }
        Ok(())
      }
      Path::Empty => Ok(()),
    }
  }
}
