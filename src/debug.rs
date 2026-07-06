use crate::{ShadowTrait, Is, Wrap, Named};

use bytemuck::TransparentWrapper;
use core::fmt::{Debug, Formatter, Result};
use core::marker::PhantomData;

pub trait DebugProvider: ShadowTrait
where
    Named<Self::Impl>: Debug,
    Self::Impl: ShadowTrait,
    Self::Target: Is<Type = <Self::Impl as ShadowTrait>::Target>,
{
    type Impl;
}

impl<N> DebugProvider for N
where
    N: ShadowTrait,
    Named<N>: Debug
{
    type Impl = Self;
}

impl<NP, const ImplDeref: bool> Debug for Wrap<NP, ImplDeref>
where
    NP: DebugProvider,
    Named<NP::Impl>: Debug,
    NP::Impl: ShadowTrait,
    NP::Target: Is<Type = <NP::Impl as ShadowTrait>::Target>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let a = &self.0;
        let b = <NP::Target as Is>::to_ref_right(a);
        let c: &Named<NP::Impl> = Named::<NP::Impl>::wrap_ref(b);
        Named::<NP::Impl>::fmt(c, f)
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
