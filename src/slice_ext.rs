pub trait SliceExt<T> {
  fn get_array<const N: usize>(&self, offset: usize) -> &[T; N];

  fn get_array_mut<const N: usize>(&mut self, offset: usize) -> &mut [T; N];

  fn split_array<const N: usize>(&self) -> (&[T; N], &[T]);
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
