use crate::prelude::*;

pub mod prelude {
    pub use super::OptMutExt;
}

pub trait OptMutExt<T> {
    fn opt_ref(&self) -> Option<&T>;
    fn opt_mut(&mut self) -> Option<Mut<T>>;
}

impl<T> OptMutExt<T> for Option<Mut<'_, T>> {
    fn opt_ref(&self) -> Option<&T> {
        self.as_ref().map(|m| m.as_ref())
    }

    fn opt_mut(&mut self) -> Option<Mut<T>> {
        self.as_mut().map(|m| m.reborrow())
    }
}
