use core::marker::PhantomData;

use type_tricks::{ShadowTrait, Wrap};

pub trait ShadowToString<T: ?Sized>: ShadowTrait<T> {
    fn to_string(this: &T) -> String;
}

pub struct DefaultToString<T: ToString + ?Sized>(PhantomData<T>);
impl<T: ToString + ?Sized> ShadowTrait<T> for DefaultToString<T> {}
impl<T: ToString + ?Sized> ShadowToString<T> for DefaultToString<T> {
    fn to_string(this: &T) -> String {
        <T as ToString>::to_string(this)
    }
}

// with Wrap<T, N> impl external trait is not allowed
/*
pub struct ToStringSelector<T: ?Sized, N: ShadowToString<T>>(PhantomData<T>, PhantomData<N>);
impl<T: ?Sized, N: ShadowToString<T>> ShadowTrait<T> for ToStringSelector<T, N> {}
// Because we cannot write `impl<N: ShadowToString> ToString for Wrap<N>`
impl<T: ?Sized, N: ShadowToString<T>> ToString for Wrap<T, ToStringSelector<T, N>> {
    fn to_string(&self) -> String {
        N::to_string(&self.1)
    }
}
*/