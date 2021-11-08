use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

pub enum Bound<'a, T: 'a> {
  Excluded(&'a T),
  Included(&'a T),
  Unbounded,
}

pub trait RangeArgument<T> {
  fn start(&self) -> Bound<T>;
  fn end(&self) -> Bound<T>;
}

// ..
impl<T> RangeArgument<T> for RangeFull {
  fn start(&self) -> Bound<T> {
    Bound::Unbounded
  }

  fn end(&self) -> Bound<T> {
    Bound::Unbounded
  }
}

// start..end
impl<T> RangeArgument<T> for Range<T> {
  fn start(&self) -> Bound<T> {
    Bound::Included(&self.start)
  }

  fn end(&self) -> Bound<T> {
    Bound::Excluded(&self.end)
  }
}

// start..=end
impl<T> RangeArgument<T> for RangeInclusive<T> {
  fn start(&self) -> Bound<T> {
    Bound::Included(RangeInclusive::start(self))
  }

  fn end(&self) -> Bound<T> {
    Bound::Included(RangeInclusive::end(self))
  }
}

// start..
impl<T> RangeArgument<T> for RangeFrom<T> {
  fn start(&self) -> Bound<T> {
    Bound::Included(&self.start)
  }

  fn end(&self) -> Bound<T> {
    Bound::Unbounded
  }
}

// ..end
impl<T> RangeArgument<T> for RangeTo<T> {
  fn start(&self) -> Bound<T> {
    Bound::Unbounded
  }

  fn end(&self) -> Bound<T> {
    Bound::Excluded(&self.end)
  }
}

// ..=end
impl<T> RangeArgument<T> for RangeToInclusive<T> {
  fn start(&self) -> Bound<T> {
    Bound::Unbounded
  }

  fn end(&self) -> Bound<T> {
    Bound::Included(&self.end)
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
