use crate::{ShadowTrait, Named, Wrap, wrap_one_tag_multiple_types::WrapOneTagMultipleTypes};

use bytemuck::TransparentWrapper;
use core::fmt::{Display, Formatter, Result};
use core::marker::PhantomData;

pub trait DisplayProvider: ShadowTrait
where
    Named<Self::Impl>: Display,
    Self::Impl: ShadowTrait<Target = Self::Target>,
{
    type Impl;
}

impl<N> DisplayProvider for N
where
    Named<N>: Display,
    N: ShadowTrait,
{
    type Impl = Self;
}

impl<NP, const IMPL_DEREF: bool> Display for Wrap<NP, IMPL_DEREF>
where
    NP: DisplayProvider,
    Named<NP::Impl>: Display,
    NP::Impl: ShadowTrait<Target = NP::Target>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Named::<NP::Impl>::fmt(Named::<NP::Impl>::wrap_ref(&self.0), f)
    }
}

#[fundamental]
pub trait DisplayProvider1<T>
where
    T: ?Sized,
    Named<Self::Impl>: Display,
    Self::Impl: ShadowTrait<Target = T>,
{
    type Impl;
}

impl<T, N> DisplayProvider1<T> for N
where
    T: ?Sized,
    Named<N>: Display,
    N: ShadowTrait<Target = T>,
{
    type Impl = Self;
}

impl<T, NP> Display for WrapOneTagMultipleTypes<T, NP>
where
    T: ?Sized,
    NP: DisplayProvider1<T>,
    Named<NP::Impl>: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Named::<NP::Impl>::fmt(Named::<NP::Impl>::wrap_ref(&self.1), f)
    }
}

pub struct DefaultDisplay<T: Display + ?Sized>(PhantomData<T>);
impl<T: Display + ?Sized> ShadowTrait for DefaultDisplay<T> {
    type Target = T;
}
impl<T: Display + ?Sized> Display for Named<DefaultDisplay<T>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        <T as Display>::fmt(&self.0, f)
    }
}
