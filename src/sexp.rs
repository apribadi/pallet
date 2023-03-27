use crate::prelude::*;

#[derive(Clone)]
pub enum Sexp {
  Sym(String),
  Seq(Box<[Sexp]>),
}

impl Sexp {
  pub fn sym(a: &str) -> Self {
    Self::Sym(a.to_string())
  }
}

impl fmt::Display for Sexp {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Sym(a) => {
        // TODO: escape
        write!(f, "{}", a)?;
      }
      Self::Seq(a) => {
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
