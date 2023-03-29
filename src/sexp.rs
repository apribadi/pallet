use crate::prelude::*;

#[derive(Clone)]
pub enum Sexp {
  Atom(String),
  List(Box<[Sexp]>),
}

impl Sexp {
  pub fn from_atom(a: &str) -> Self {
    Self::Atom(a.to_string())
  }
}

pub trait ToSexp {
  fn to_sexp(&self) -> Sexp;
}

impl fmt::Display for Sexp {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Atom(a) => {
        // TODO: escape
        write!(f, "{}", a)?;
      }
      Self::List(a) => {
        write!(f, "(")?;
        if let Some((x, y)) = a.split_first() {
          write!(f, "{}", x)?;
          for z in y.iter() { write!(f, " {}", z)? }
        }
        write!(f, ")")?;
      }
    }

    Ok(())
  }
}
