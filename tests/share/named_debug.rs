use core::marker::PhantomData;
use core::fmt::{Debug, Formatter, Result};

use type_tricks::{NamedImplBase, Wrap};

pub trait NamedDebug: NamedImplBase {
    fn fmt(this: &Self::Target, f: &mut Formatter<'_>) -> Result;
}

pub struct DebugImpl<N: NamedDebug>(PhantomData<N>);
impl<N> NamedImplBase for DebugImpl<N>
where
    N: NamedDebug,
{
    type Target = N::Target;
}

impl<N> Debug for Wrap<DebugImpl<N>>
    where N: NamedDebug,
          N::Target: Sized
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        N::fmt(&self.value, f)
    }
}

pub struct DefaultDebug<T: Debug>(PhantomData<T>);
impl<T: Debug> NamedImplBase for DefaultDebug<T> {
    type Target = T;
}
impl<T: Debug> NamedDebug for DefaultDebug<T> {
    fn fmt(this: &Self::Target, f: &mut Formatter<'_>) -> Result {
        this.fmt(f)
    }
}
