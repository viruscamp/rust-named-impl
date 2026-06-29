use core::marker::PhantomData;

use type_tricks::{NamedImplBase, Wrap};

pub trait NamedToString: NamedImplBase {
    fn to_string(this: &Self::Target) -> String;
}

pub struct ToStringImpl<N: NamedToString>(PhantomData<N>);
impl<N> NamedImplBase for ToStringImpl<N>
where
    N: NamedToString,
{
    type Target = N::Target;
}

impl<N: NamedToString> ToString for Wrap<ToStringImpl<N>>
where
    N: NamedToString,
    N::Target: Sized,
{
    fn to_string(&self) -> String {
        N::to_string(&self.value)
    }
}

pub struct DefaultToString<T: ToString>(PhantomData<T>);
impl<T: ToString> NamedImplBase for DefaultToString<T> {
    type Target = T;
}
impl<T: ToString> NamedToString for DefaultToString<T> {
    fn to_string(this: &Self::Target) -> String {
        this.to_string()
    }
}
