#[derive(
  Clone,
  Copy,
  Debug,
  Eq,
  Hash,
  Ord,
  PartialEq,
  PartialOrd
)]
pub struct Phantom<T: ?Sized> {
  phantom_data: core::marker::PhantomData<T>,
}

pub type Contravariant<T> = Phantom<fn(T)>;

pub type Covariant<T> = Phantom<T>;

pub type Invariant<T> = Phantom<fn(T) -> T>;

pub type Lifetime<'a> = Phantom<&'a ()>;

impl<T: ?Sized> Phantom<T> {
  #[inline(always)]
  pub const fn new() -> Self {
    Self { phantom_data: core::marker::PhantomData }
  }
}

impl<T: ?Sized> Default for Phantom<T> { fn default() -> Self { Self::new() } }

impl<T: ?Sized> Unpin for Phantom<T> { }

impl<T: ?Sized> core::panic::RefUnwindSafe for Phantom<T> { }

impl<T: ?Sized> core::panic::UnwindSafe for Phantom<T> { }

unsafe impl<T: ?Sized> Send for Phantom<T> { }

unsafe impl<T: ?Sized> Sync for Phantom<T> { }
