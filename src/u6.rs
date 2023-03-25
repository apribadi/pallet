#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct u6(u8);

impl From<u8> for u6 {
  #[inline(always)]
  fn from(x: u8) -> Self {
    Self(x & 0x3f)
  }
}

impl From<u6> for u8 {
  #[inline(always)]
  fn from(u6(x): u6) -> Self {
    x
  }
}
