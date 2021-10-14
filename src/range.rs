use std::ops::{Range, RangeFrom, RangeFull, RangeTo};

pub enum Bound<'a, T: 'a> {
  Excluded(&'a T),
  Included(&'a T),
  Unbounded,
}

pub trait RangeArgument<T> {
  fn start(&self) -> Bound<T>;
  fn end(&self) -> Bound<T>;
}

impl<T> RangeArgument<T> for Range<T> {
  fn start(&self) -> Bound<T> {
    Bound::Included(&self.start)
  }

  fn end(&self) -> Bound<T> {
    Bound::Excluded(&self.end)
  }
}

impl<T> RangeArgument<T> for RangeFrom<T> {
  fn start(&self) -> Bound<T> {
    Bound::Included(&self.start)
  }

  fn end(&self) -> Bound<T> {
    Bound::Unbounded
  }
}

impl<T> RangeArgument<T> for RangeTo<T> {
  fn start(&self) -> Bound<T> {
    Bound::Unbounded
  }

  fn end(&self) -> Bound<T> {
    Bound::Excluded(&self.end)
  }
}

impl<T> RangeArgument<T> for RangeFull {
  fn start(&self) -> Bound<T> {
    Bound::Unbounded
  }

  fn end(&self) -> Bound<T> {
    Bound::Unbounded
  }
}

impl RangeArgument<usize> for usize {
  fn start(&self) -> Bound<usize> {
    Bound::Included(self)
  }

  fn end(&self) -> Bound<usize> {
    Bound::Included(self)
  }
}
