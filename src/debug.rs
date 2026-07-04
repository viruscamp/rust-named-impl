use crate::{ShadowTrait, is::Is};

use core::fmt::{Debug, Formatter, Result};
use std::marker::PhantomData;

pub trait ShadowDebug: ShadowTrait {
    fn fmt(this: &Self::Target, f: &mut Formatter<'_>) -> Result;
}

pub trait ShadowDebugProvider: ShadowTrait
    where <Self::Impl as ShadowTrait>::Target: Is<Type = Self::Target>
{
    type Impl: ShadowDebug;
}

impl<N: ShadowDebug> ShadowDebugProvider for N {
    type Impl = Self;
}

// https://github.com/rust-lang/rust/issues/124449
impl<NP: ShadowDebugProvider, const ImplDeref: bool> Debug for crate::Wrap<NP, ImplDeref> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        NP::Impl::fmt(Is::to_ref_left(&self.0), f)
    }
}

pub struct DefaultDebug<T: Debug>(PhantomData<T>);
impl<T: Debug> ShadowTrait for DefaultDebug<T> {
    type Target = T;
}
impl<T: Debug> ShadowDebug for DefaultDebug<T> {
    fn fmt(this: &Self::Target, f: &mut Formatter<'_>) -> Result {
        <T as Debug>::fmt(this, f)
    }
}
