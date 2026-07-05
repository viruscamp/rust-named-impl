use crate::{ShadowTrait, Wrap};

use core::fmt::{Debug, Formatter, Result};
use std::marker::PhantomData;

pub trait ShadowDebug<T: ?Sized>: ShadowTrait<T> {
    fn fmt(this: &T, f: &mut Formatter<'_>) -> Result;
}

pub trait ShadowDebugProvider<T: ?Sized>: ShadowTrait<T> {
    type Impl: ShadowDebug<T>;
}

impl<T: ?Sized, N: ShadowDebug<T>> ShadowDebugProvider<T> for N {
    type Impl = Self;
}

// https://github.com/rust-lang/rust/issues/124449
impl<T: ?Sized, NP: ShadowDebugProvider<T>, const ImplDeref: bool>
Debug for Wrap<T, NP, ImplDeref> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        NP::Impl::fmt(&self.1, f)
    }
}

pub struct DefaultDebug<T: Debug + ?Sized>(PhantomData<T>);

impl<T: Debug + ?Sized> ShadowTrait<T> for DefaultDebug<T> {}

impl<T: Debug + ?Sized> ShadowDebug<T> for DefaultDebug<T> {
    fn fmt(this: &T, f: &mut Formatter<'_>) -> Result {
        <T as Debug>::fmt(this, f)
    }
}
