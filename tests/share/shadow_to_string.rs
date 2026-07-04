use core::marker::PhantomData;

use type_tricks::{ShadowTrait, Wrap};

pub trait ShadowToString: ShadowTrait {
    fn to_string(this: &Self::Target) -> String;
}

pub struct DefaultToString<T: ToString>(PhantomData<T>);
impl<T: ToString> ShadowTrait for DefaultToString<T> {
    type Target = T;
}
impl<T: ToString> ShadowToString for DefaultToString<T> {
    fn to_string(this: &Self::Target) -> String {
        <T as ToString>::to_string(this)
    }
}

pub struct ToStringSelector<N: ShadowToString>(PhantomData<N>);
impl<N: ShadowToString> ShadowTrait for ToStringSelector<N> {
    type Target = N::Target;
}
// Because we cannot write `impl<N: ShadowToString> ToString for Wrap<N>`
impl<N: ShadowToString> ToString for Wrap<ToStringSelector<N>> {
    fn to_string(&self) -> String {
        N::to_string(&self.0)
    }
}
