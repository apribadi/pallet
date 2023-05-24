use crate::prelude::*;

pub struct ByteCursor<'a>(&'a [u8]);

impl<'a> ByteCursor<'a> {
  pub fn new(t: &'a [u8]) -> Self {
    Self(t)
  }

  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  #[inline(always)]
  pub fn pop_array<const N: usize>(&mut self) -> &'a [u8; N] {
    let (x, y) = self.0.split_array();
    self.0 = y;
    x
  }

  pub fn pop_slice(&mut self, k: usize) -> &'a [u8] {
    let (x, y) = self.0.split_at(k);
    self.0 = y;
    x
  }

  pub fn pop_u8(&mut self) -> u8 {
    u8::from_le_bytes(*self.pop_array())
  }

  pub fn pop_u16(&mut self) -> u16 {
    u16::from_le_bytes(*self.pop_array())
  }

  pub fn pop_u32(&mut self) -> u32 {
    u32::from_le_bytes(*self.pop_array())
  }

  pub fn pop_u64(&mut self) -> u64 {
    u64::from_le_bytes(*self.pop_array())
  }
}

pub struct ByteBuf(Vec<u8>);

impl ByteBuf {
  pub fn new() -> Self {
    Self(Vec::new())
  }

  #[inline(always)]
  pub fn put_array<const N: usize>(&mut self, value: &[u8; N]) {
    self.0.extend_from_slice(value)
  }

  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn put_u8(&mut self, value: u8) {
    self.put_array(&value.to_le_bytes())
  }

  pub fn put_u16(&mut self, value: u16) {
    self.put_array(&value.to_le_bytes())
  }

  pub fn put_u32(&mut self, value: u32) {
    self.put_array(&value.to_le_bytes())
  }

  pub fn put_u64(&mut self, value: u64) {
    self.put_array(&value.to_le_bytes())
  }

  pub fn set_u8(&mut self, offset: usize, value: u8) {
    *self.0.get_array_mut(offset) = value.to_le_bytes()
  }

  pub fn set_u16(&mut self, offset: usize, value: u16) {
    *self.0.get_array_mut(offset) = value.to_le_bytes()
  }

  pub fn set_u32(&mut self, offset: usize, value: u32) {
    *self.0.get_array_mut(offset) = value.to_le_bytes()
  }

  pub fn set_u64(&mut self, offset: usize, value: u64) {
    *self.0.get_array_mut(offset) = value.to_le_bytes()
  }
}
