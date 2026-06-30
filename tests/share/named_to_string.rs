use core::marker::PhantomData;

use type_tricks::{NamedImplBase, Wrap};

pub trait NamedToString: NamedImplBase {
    fn to_string(this: &Self::Target) -> String;
}

pub struct DefaultToString<T: ToString>(PhantomData<T>);
impl<T: ToString> NamedImplBase for DefaultToString<T> {
    type Target = T;
}
impl<T: ToString> NamedToString for DefaultToString<T> {
    fn to_string(this: &Self::Target) -> String {
        <T as ToString>::to_string(this)
    }
}

pub struct ToStringSelector<N: NamedToString>(PhantomData<N>);
impl<N> NamedImplBase for ToStringSelector<N>
where
    N: NamedToString,
{
    type Target = N::Target;
}
// Because we cannot write `impl<N: NamedToString> ToString for Wrap<N>`
impl<N: NamedToString> ToString for Wrap<ToStringSelector<N>>
where
    N: NamedToString,
    N::Target: Sized,
{
    fn to_string(&self) -> String {
        N::to_string(&self.value)
    }
}
