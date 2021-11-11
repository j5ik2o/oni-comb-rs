use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Path {
  RootlessPath {
    type_name: &'static str,
    parts: Vec<String>,
  },
  AbemptyPath {
    type_name: &'static str,
    parts: Vec<String>,
  },
  AbsolutePath {
    type_name: &'static str,
    parts: Vec<String>,
  },
  NoSchemePath {
    type_name: &'static str,
    parts: Vec<String>,
  },
  EmptyPath {
    type_name: &'static str,
  },
}

impl Default for Path {
  fn default() -> Self {
    Path::of_empty()
  }
}

impl fmt::Display for Path {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    // let root = match self {
    //   Path::RootlessPath { .. } | Path::NoSchemePath { .. } | Path::EmptyPath { .. } => "",
    //   _ => "/",
    // };
    write!(f, "{}", self.parts().join(""))
  }
}

impl Path {
  pub fn of_rootless_from_strs(parts: &[&str]) -> Self {
    Path::RootlessPath {
      type_name: "rootless_path",
      parts: parts.into_iter().map(|e| e.to_string()).collect::<Vec<_>>(),
    }
  }

  pub fn of_rootless_from_strings(parts: &[String]) -> Self {
    Path::RootlessPath {
      type_name: "rootless_path",
      parts: Vec::from(parts),
    }
  }

  pub fn of_abempty_from_strs(parts: &[&str]) -> Self {
    Path::AbemptyPath {
      type_name: "abempty_path",
      parts: parts.into_iter().map(|e| e.to_string()).collect::<Vec<_>>(),
    }
  }

  pub fn of_abempty_from_strings(parts: &[String]) -> Self {
    Path::AbemptyPath {
      type_name: "abempty_path",
      parts: Vec::from(parts),
    }
  }

  pub fn of_absolute_from_strs(parts: &[&str]) -> Self {
    Path::AbsolutePath {
      type_name: "absolute_path",
      parts: parts.into_iter().map(|e| e.to_string()).collect::<Vec<_>>(),
    }
  }

  pub fn of_absolute_from_strings(parts: &[String]) -> Self {
    Path::AbsolutePath {
      type_name: "absolute_path",
      parts: Vec::from(parts),
    }
  }

  pub fn of_no_scheme_from_strs(parts: &[&str]) -> Self {
    Path::NoSchemePath {
      type_name: "no_scheme_path",
      parts: parts.into_iter().map(|e| e.to_string()).collect::<Vec<_>>(),
    }
  }

  pub fn of_no_scheme_from_strings(parts: &[String]) -> Self {
    Path::NoSchemePath {
      type_name: "no_scheme_path",
      parts: Vec::from(parts),
    }
  }

  pub fn of_empty() -> Self {
    Path::EmptyPath {
      type_name: "empty_path",
    }
  }

  pub fn type_name(&self) -> &'static str {
    match self {
      &Path::RootlessPath { type_name, .. } => type_name,
      &Path::AbemptyPath { type_name, .. } => type_name,
      &Path::AbsolutePath { type_name, .. } => type_name,
      &Path::NoSchemePath { type_name, .. } => type_name,
      &Path::EmptyPath { type_name } => type_name,
    }
  }

  pub fn parts(&self) -> &Vec<String> {
    static EMPTY_PARTS: Vec<String> = vec![];
    match self {
      Path::RootlessPath { parts, .. } => parts,
      Path::AbemptyPath { parts, .. } => parts,
      Path::AbsolutePath { parts, .. } => parts,
      Path::NoSchemePath { parts, .. } => parts,
      Path::EmptyPath { .. } => &EMPTY_PARTS,
    }
  }

  pub fn is_empty(&self) -> bool {
    self.parts().is_empty()
  }

  pub fn non_empty(&self) -> bool {
    !self.is_empty()
  }

  pub fn with_parts(&mut self, parts: Vec<String>) {
    self.add_parts(parts)
  }

  pub fn to_rootless(&self) -> Path {
    Path::of_rootless_from_strings(&self.parts().clone())
  }

  pub fn to_absolute(&self) -> Path {
    Path::of_absolute_from_strings(&self.parts().clone())
  }

  pub fn add_part(&mut self, part: String) {
    let parts_opt = match self {
      Path::RootlessPath { parts, .. } => Some(parts),
      Path::AbemptyPath { parts, .. } => Some(parts),
      Path::AbsolutePath { parts, .. } => Some(parts),
      Path::NoSchemePath { parts, .. } => Some(parts),
      Path::EmptyPath { .. } => None,
    };
    match parts_opt {
      Some(parts) => {
        parts.push(part);
      }
      None => (),
    }
  }

  pub fn add_parts(&mut self, parts: Vec<String>) {
    for x in parts {
      self.add_part(x)
    }
  }
}
