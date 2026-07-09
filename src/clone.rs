use crate::{ShadowTrait, Named, Wrap, wrap_one_tag_multiple_types::WrapOneTagMultipleTypes};

use bytemuck::TransparentWrapper;
use core::marker::PhantomData;

pub trait CloneProvider: ShadowTrait
where
    Named<Self::Impl>: Clone,
    Self::Impl: ShadowTrait<Target = Self::Target>,
{
    type Impl;
}

impl<N> CloneProvider for N
where
    Named<N>: Clone,
    N: ShadowTrait,
{
    type Impl = Self;
}

impl<NP, const IMPL_DEREF: bool> Clone for Wrap<NP, IMPL_DEREF>
where
    NP: CloneProvider,
    Named<NP::Impl>: Clone,
    NP::Impl: ShadowTrait<Target = NP::Target>,
    NP::Target: Sized,
{
    fn clone(&self) -> Self {
        let a = &self.0;
        let b = Named::<NP::Impl>::wrap_ref(a);
        let c = b.clone();
        Self::new(c.0)
    }
}

#[fundamental]
pub trait CloneProvider1<T>
where
    Named<Self::Impl>: Clone,
    Self::Impl: ShadowTrait<Target = T>,
{
    type Impl;
}

impl<T, N> CloneProvider1<T> for N
where
    Named<N>: Clone,
    N: ShadowTrait<Target = T>,
{
    type Impl = Self;
}

impl<T, NP> Clone for WrapOneTagMultipleTypes<T, NP>
where
    NP: CloneProvider1<T>,
    Named<NP::Impl>: Clone,
{
    fn clone(&self) -> Self {
        let a = &self.1;
        let b = Named::<NP::Impl>::wrap_ref(a);
        let c = b.clone();
        Self::new(c.0)
    }
}

pub struct DefaultClone<T: Clone>(PhantomData<T>);
impl<T: Clone> ShadowTrait for DefaultClone<T> {
    type Target = T;
}
impl<T: Clone> Clone for Named<DefaultClone<T>> {
    fn clone(&self) -> Self {
        Self(T::clone(&self.0))
    }
}
