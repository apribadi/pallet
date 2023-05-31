use crate::prelude::*;

#[inline(always)]
pub fn field<const N: usize, const I: usize>(a: &[u8]) -> &[u8] {
  assert!(I < N);

  let u =
    if I == 0 {
      4 * N
    } else {
      4 * N + a.get_u32(4 * I - 4) as usize
    };

  let v =
    if I == N - 1 {
      a.len()
    } else {
      4 * N + a.get_u32(4 * I) as usize
    };

  &a[u .. v]
}

#[inline(always)]
pub fn pop_u8<'a, 'b>(a: &'a mut &'b [u8]) -> u8 {
  let (b, c) = a.split_array::<1>();
  *a = c;
  u8::from_le_bytes(*b)
}

#[inline(always)]
pub fn pop_32<'a, 'b>(a: &'a mut &'b [u8]) -> u32 {
  let (b, c) = a.split_array::<4>();
  *a = c;
  u32::from_le_bytes(*b)
}

#[inline(always)]
pub fn iter_variable_sized<'a>(a: &'a [u8]) -> impl Iterator<Item = &'a [u8]> {
  IterVariableSized(a)
}

#[inline(always)]
pub fn iter_constant_sized<'a, const N: usize>(a: &'a [u8]) -> impl Iterator<Item = &'a [u8; N]> {
  IterConstantSized(a)
}

#[derive(Clone, Copy)]
pub struct IterVariableSized<'a>(&'a [u8]);

impl<'a> Iterator for IterVariableSized<'a> {
  type Item = &'a [u8];

  #[inline(always)]
  fn next(&mut self) -> Option<&'a [u8]> {
    if self.0.len() == 0 {
      return None;
    }

    let k = self.0.get_u32(0) as usize;
    let r = &self.0[4 .. 4 + k];

    self.0 = &self.0[4 + k ..];

    Some(r)
  }
}

#[derive(Clone, Copy)]
pub struct IterConstantSized<'a, const N: usize>(&'a [u8]);

impl<'a, const N: usize> Iterator for IterConstantSized<'a, N> {
  type Item = &'a [u8; N];

  #[inline(always)]
  fn next(&mut self) -> Option<&'a [u8; N]> {
    if self.0.len() == 0 {
      return None;
    }

    let (x, y) = self.0.split_array::<N>();

    self.0 = y;

    Some(x)
  }
}

/*

#[derive(Clone, Copy)]
pub struct BC<'a>(&'a [u8]);

pub fn new<'a, T>(slice: &'a [u8]) -> BC<'a, T> {
  BC(slice, Phantom::new())
}

pub fn field<'a, T, U>(bc: BC<'a, T>, n: usize, i: usize) -> BC<'a, U> {
  assert!(i < n);
  assert!(bc.0.len() >= 4 * i - 4);

  let u =
    if i == 0 {
      4 * n
    } else {
      4 * n + bc.0.get_u32(4 * i - 4) as usize
    };

  let v =
    if i == n - 1 {
      bc.0.len()
    } else {
      4 * n + bc.0.get_u32(4 * i) as usize
    };

  let r = &bc.0[u .. v];

  new(r)
}

pub fn next<'a, T, U>(bc: &mut BC<'a, T>) -> Option<BC<'a, U>> {
  if bc.0.len() == 0 { return None; }

  let n = bc.0.get_u32(0) as usize;
  let r = &bc.0[4 .. 4 + n];

  bc.0 = &bc.0[4 + n ..];

  Some(new(r))
}

impl Ty {
  pub fn decode(t: u8) -> Option<Self> {
    if (t as usize) >= Self::VARIANT_COUNT { return None; }
    Some(unsafe { core::mem::transmute::<u8, Self>(t) })
  }
}

impl InstTag {
  pub fn decode(t: u8) -> Option<Self> {
    if (t as usize) >= Self::VARIANT_COUNT { return None; }
    Some(unsafe { core::mem::transmute::<u8, Self>(t) })
  }
}

impl Op11 {
  pub fn decode(t: u8) -> Option<Self> {
    if (t as usize) >= Self::VARIANT_COUNT { return None; }
    Some(unsafe { core::mem::transmute::<u8, Self>(t) })
  }
}

impl Op21 {
  pub fn decode(t: u8) -> Option<Self> {
    if (t as usize) >= Self::VARIANT_COUNT { return None; }
    Some(unsafe { core::mem::transmute::<u8, Self>(t) })
  }
}
*/
