use once_cell::sync::OnceCell;

#[derive(Debug)]
pub struct LateInit<T> {
  cell: OnceCell<T>,
}

impl<T> LateInit<T> {
  pub fn init(&self, value: T) {
    assert!(self.cell.set(value).is_ok())
  }
  pub const fn new() -> LateInit<T> {
    LateInit {
      cell: OnceCell::new(),
    }
  }
}

impl<T> Default for LateInit<T> {
  fn default() -> Self {
    LateInit::new()
  }
}

impl<T> std::ops::Deref for LateInit<T> {
  type Target = T;
  #[inline]
  fn deref(&self) -> &T {
    unsafe { self.cell.get_unchecked() }
  }
}
