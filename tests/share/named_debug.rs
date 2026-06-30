use core::marker::PhantomData;
use core::fmt::{Debug, Formatter, Result};

use type_tricks::{NamedImplBase, Wrap};

pub trait NamedDebug: NamedImplBase {
    fn fmt(this: &Self::Target, f: &mut Formatter<'_>) -> Result;
}

pub struct DefaultDebug<T: Debug>(PhantomData<T>);
impl<T: Debug> NamedImplBase for DefaultDebug<T> {
    type Target = T;
}
impl<T: Debug> NamedDebug for DefaultDebug<T> {
    fn fmt(this: &Self::Target, f: &mut Formatter<'_>) -> Result {
        <T as Debug>::fmt(this, f)
    }
}

pub struct DebugSelector<N: NamedDebug>(PhantomData<N>);
impl<N> NamedImplBase for DebugSelector<N>
where
    N: NamedDebug,
{
    type Target = N::Target;
}
// Because we cannot write `impl<N: NamedDebug> Debug for Wrap<N>`
impl<N> Debug for Wrap<DebugSelector<N>>
    where N: NamedDebug,
          N::Target: Sized
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        N::fmt(&self.value, f)
    }
}
