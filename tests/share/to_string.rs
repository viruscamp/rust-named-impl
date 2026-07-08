use core::marker::PhantomData;
use bytemuck::TransparentWrapper;
use named_impl::{Named, ShadowTrait, Wrap};

pub struct DefaultToString<T: ToString>(PhantomData<T>);
impl<T: ToString> ShadowTrait for DefaultToString<T> {
    type Target = T;
}
impl<T: ToString> ToString for Named<DefaultToString<T>> {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

// this is useless, becasue we cannot use it in external crate
pub trait ToStringProvider: ShadowTrait
where
    Named<Self::Impl>: ToString,
    Self::Impl: ShadowTrait<Target = Self::Target>,
{
    type Impl;
}

impl<N> ToStringProvider for N
where
    Named<N>: ToString,
    N: ShadowTrait,
{
    type Impl = Self;
}

// failed
//impl<NP: ToStringProvider, const ImplDeref: bool> ToString for Wrap<NP, ImplDeref>{}

pub struct ToStringSelector<N>(PhantomData<N>)
where N: ShadowTrait, Named<N>: ToString;
impl<N> ShadowTrait for ToStringSelector<N>
where N: ShadowTrait, Named<N>: ToString
{
    type Target = N::Target;
}
// Because we cannot write `impl<N> ToString for Wrap<N>`
impl<N> ToString for Wrap<ToStringSelector<N>>
where N: ShadowTrait, Named<N>: ToString
{
    fn to_string(&self) -> String {
        Named::<N>::to_string(Named::<N>::wrap_ref(&self.0))
    }
}

// failed
//impl<T, NP> ToString for named_impl::wrap1::Wrap1<T, NP> {}
