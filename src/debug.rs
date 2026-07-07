use crate::{ShadowTrait, Named, Wrap, wrap1::Wrap1};

use bytemuck::TransparentWrapper;
use core::fmt::{Debug, Formatter, Result};
use core::marker::PhantomData;

pub trait DebugProvider: ShadowTrait
where
    Named<Self::Impl>: Debug,
    Self::Impl: ShadowTrait<Target = Self::Target>,
{
    type Impl;
}

impl<N> DebugProvider for N
where
    Named<N>: Debug,
    N: ShadowTrait,
{
    type Impl = Self;
}

impl<NP, const ImplDeref: bool> Debug for Wrap<NP, ImplDeref>
where
    NP: DebugProvider,
    Named<NP::Impl>: Debug,
    NP::Impl: ShadowTrait<Target = NP::Target>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Named::fmt(Named::wrap_ref(&self.0), f)
    }
}

pub trait DebugProvider1<T>
where
    T: ?Sized,
    Named<Self::Impl>: Debug,
    Self::Impl: ShadowTrait<Target = T>,
{
    type Impl;
}

impl<T, N> DebugProvider1<T> for N
where
    T: ?Sized,
    Named<N>: Debug,
    N: ShadowTrait<Target = T>,
{
    type Impl = Self;
}

impl<T, NP> Debug for Wrap1<T, NP>
where
    T: ?Sized,
    NP: DebugProvider1<T>,
    Named<NP::Impl>: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Named::fmt(Named::wrap_ref(&self.1), f)
    }
}

pub struct DefaultDebug<T: Debug + ?Sized>(PhantomData<T>);
impl<T: Debug + ?Sized> ShadowTrait for DefaultDebug<T> {
    type Target = T;
}
impl<T: Debug + ?Sized> Debug for Named<DefaultDebug<T>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        <T as Debug>::fmt(&self.0, f)
    }
}
