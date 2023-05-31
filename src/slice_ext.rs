pub trait SliceExt<T> {
  fn get_array<const N: usize>(&self, offset: usize) -> &[T; N];
  fn get_array_mut<const N: usize>(&mut self, offset: usize) -> &mut [T; N];
  fn split_array<const N: usize>(&self) -> (&[T; N], &[T]);
}

pub trait SliceRefExt<'a, T> {
  fn pop_array<const N: usize>(&mut self) -> &'a [T; N];
  fn pop_slice(&mut self, len: usize) -> &'a [T];
}

pub trait ByteSliceExt {
  fn get_u8(&self, offset: usize) -> u8;
  fn get_u16(&self, offset: usize) -> u16;
  fn get_u32(&self, offset: usize) -> u32;
  fn get_u64(&self, offset: usize) -> u64;
  fn set_u8(&mut self, offset: usize, value: u8);
  fn set_u16(&mut self, offset: usize, value: u16);
  fn set_u32(&mut self, offset: usize, value: u32);
  fn set_u64(&mut self, offset: usize, value: u64);
}

pub trait ByteSliceRefExt {
  fn pop_u8(&mut self) -> u8;
  fn pop_u16(&mut self) -> u16;
  fn pop_u32(&mut self) -> u32;
}

impl<T> SliceExt<T> for [T] {
  #[inline(always)]
  fn get_array<const N: usize>(&self, offset: usize) -> &[T; N] {
    let len = self.len();
    assert!(offset <= len && N <= len - offset);
    let p = self.as_ptr();
    let p = unsafe { p.add(offset) };
    let p = p as *const [T; N];
    unsafe { &*p }
  }

  #[inline(always)]
  fn get_array_mut<const N: usize>(&mut self, offset: usize) -> &mut [T; N] {
    let len = self.len();
    assert!(offset <= len && N <= len - offset);
    let p = self.as_mut_ptr();
    let p = unsafe { p.add(offset) };
    let p = p as *mut [T; N];
    unsafe { &mut *p }
  }

  #[inline(always)]
  fn split_array<const N: usize>(&self) -> (&[T; N], &[T]) {
    let len = self.len();
    assert!(N <= len);
    let x = self.as_ptr() as *const [T; N];
    let x = unsafe { &*x };
    let y = unsafe { self.get_unchecked(N ..) };
    (x, y)
  }
}

impl<'a, T> SliceRefExt<'a, T> for &'a [T] {
  #[inline(always)]
  fn pop_array<const N: usize>(&mut self) -> &'a [T; N] {
    let (x, y) = self.split_array();
    *self = y;
    x
  }

  #[inline(always)]
  fn pop_slice(&mut self, len: usize) -> &'a [T] {
    let (x, y) = self.split_at(len);
    *self = y;
    x
  }
}

impl ByteSliceExt for [u8] {
  #[inline(always)]
  fn get_u8(&self, offset: usize) -> u8 {
    u8::from_le_bytes(*self.get_array(offset))
  }

  #[inline(always)]
  fn get_u16(&self, offset: usize) -> u16 {
    u16::from_le_bytes(*self.get_array(offset))
  }

  #[inline(always)]
  fn get_u32(&self, offset: usize) -> u32 {
    u32::from_le_bytes(*self.get_array(offset))
  }

  #[inline(always)]
  fn get_u64(&self, offset: usize) -> u64 {
    u64::from_le_bytes(*self.get_array(offset))
  }

  #[inline(always)]
  fn set_u8(&mut self, offset: usize, value: u8) {
    *self.get_array_mut(offset) = value.to_le_bytes();
  }

  #[inline(always)]
  fn set_u16(&mut self, offset: usize, value: u16) {
    *self.get_array_mut(offset) = value.to_le_bytes();
  }

  #[inline(always)]
  fn set_u32(&mut self, offset: usize, value: u32) {
    *self.get_array_mut(offset) = value.to_le_bytes();
  }

  #[inline(always)]
  fn set_u64(&mut self, offset: usize, value: u64) {
    *self.get_array_mut(offset) = value.to_le_bytes();
  }
}

impl ByteSliceRefExt for &[u8] {
  #[inline(always)]
  fn pop_u8(&mut self) -> u8 {
    u8::from_le_bytes(*self.pop_array())
  }

  #[inline(always)]
  fn pop_u16(&mut self) -> u16 {
    u16::from_le_bytes(*self.pop_array())
  }

  #[inline(always)]
  fn pop_u32(&mut self) -> u32 {
    u32::from_le_bytes(*self.pop_array())
  }
}
