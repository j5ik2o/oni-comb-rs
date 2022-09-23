// use crate::model::config_value::ConfigValue;
//
// #[derive(Clone, Debug, PartialEq)]
// pub enum ConfigValues {
//   Single(ConfigValue),
//   Multi(Vec<ConfigValue>),
// }
//
// impl ConfigValues {
//   pub fn of_single(cv: ConfigValue) -> Self {
//     ConfigValues::Single(cv)
//   }
//
//   pub fn of_multi(cvs: Vec<ConfigValue>) -> Self {
//     ConfigValues::Multi(cvs)
//   }
//
//   pub fn push(&mut self, cv: ConfigValue) {
//     match self {
//       ConfigValues::Single(v) => *self = Self::of_multi(vec![v.clone(), cv]),
//       ConfigValues::Multi(v) => v.push(cv),
//     }
//   }
//
//   pub fn with_fallback(&mut self, other: Self) {
//     match self {
//       ConfigValues::Single(v) => {
//         let mut last = v.clone();
//         last.with_fallback(other.latest().clone());
//         *self = Self::of_multi(vec![v.clone(), last]);
//       }
//       ConfigValues::Multi(v) => {
//         let mut last = v.last().unwrap().clone();
//         last.with_fallback(other.latest().clone());
//         v.push(last);
//       }
//     }
//   }
//
//   pub fn combine(&mut self, other: Self) {
//     match other {
//       ConfigValues::Single(v) => {
//         self.push(v);
//       }
//       ConfigValues::Multi(values) => {
//         for v in values {
//           self.push(v);
//         }
//       }
//     }
//   }
//
//   pub fn oldest(&self) -> &ConfigValue {
//     match self {
//       ConfigValues::Single(v) => v,
//       ConfigValues::Multi(v) => v.first().unwrap(),
//     }
//   }
//
//   pub fn index(&self, idx: usize) -> &ConfigValue {
//     match self {
//       ConfigValues::Single(v) => v,
//       ConfigValues::Multi(v) => &v[idx],
//     }
//   }
//
//   pub fn latest(&self) -> &ConfigValue {
//     match self {
//       ConfigValues::Single(v) => v,
//       ConfigValues::Multi(v) => v.last().unwrap(),
//     }
//   }
//
//   pub fn prev_latest(&self) -> Option<&ConfigValue> {
//     match self {
//       ConfigValues::Single(_v) => None,
//       ConfigValues::Multi(v) => Some(&v[v.len() - 2]),
//     }
//   }
// }
