use crate::{ShadowTrait, is::Is};

use core::fmt::{Display, Formatter, Result};
use core::marker::PhantomData;

pub trait ShadowDisplay: ShadowTrait {
    fn fmt(this: &Self::Target, f: &mut Formatter<'_>) -> Result;
}

pub trait ShadowDisplayProvider: ShadowTrait
    where <Self::Impl as ShadowTrait>::Target: Is<Type = Self::Target>
{
    type Impl: ShadowDisplay;
}

impl<N: ShadowDisplay> ShadowDisplayProvider for N {
    type Impl = Self;
}

impl<NP: ShadowDisplayProvider, const ImplDeref: bool> Display for crate::Wrap<NP, ImplDeref> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        NP::Impl::fmt(Is::to_ref_left(&self.0), f)
    }
}

pub struct DefaultDisplay<T: Display + ?Sized>(PhantomData<T>);
impl<T: Display + ?Sized> ShadowTrait for DefaultDisplay<T> {
    type Target = T;
}
impl<T: Display + ?Sized> ShadowDisplay for DefaultDisplay<T> {
    fn fmt(this: &Self::Target, f: &mut Formatter<'_>) -> Result {
        <T as Display>::fmt(this, f)
    }
}
