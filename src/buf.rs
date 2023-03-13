use crate::prelude::*;

pub struct ByteCursor<'a>(&'a [u8]);

impl<'a> ByteCursor<'a> {
  #[inline(always)]
  pub fn new(t: &'a [u8]) -> Self {
    Self(t)
  }

  #[inline(always)]
  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  #[inline(always)]
  pub fn pop_array<const N: usize>(&mut self) -> &'a [u8; N] {
    let (x, y) = self.0.split_array();
    self.0 = y;
    x
  }

  #[inline(always)]
  pub fn pop_slice(&mut self, k: usize) -> &'a [u8] {
    let (x, y) = self.0.split_at(k);
    self.0 = y;
    x
  }

  #[inline(always)]
  pub fn pop_u8(&mut self) -> u8 {
    u8::from_le_bytes(*self.pop_array())
  }

  #[inline(always)]
  pub fn pop_u16(&mut self) -> u16 {
    u16::from_le_bytes(*self.pop_array())
  }

  #[inline(always)]
  pub fn pop_u32(&mut self) -> u32 {
    u32::from_le_bytes(*self.pop_array())
  }

  #[inline(always)]
  pub fn pop_u64(&mut self) -> u64 {
    u64::from_le_bytes(*self.pop_array())
  }
}
