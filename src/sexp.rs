use crate::prelude::*;

#[derive(Clone, Copy)]
pub enum Sexp<'a> {
  Sym(&'a str),
  Seq(&'a [Sexp<'a>]),
}

impl<'a> fmt::Display for Sexp<'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Sym(s) => {
        // TODO: escape
        write!(f, "{}", s)?;
      }
      Self::Seq(s) => {
        write!(f, "(")?;
        if let Some((x, y)) = s.split_first() {
          write!(f, "{}", x)?;
          for z in y.iter() { write!(f, " {}", z)? }
        }
        write!(f, ")")?;
      }
    }

    Ok(())
  }
}
