#![no_std]
#![feature(fundamental)]
#![feature(with_negative_coherence)]

// https://internals.rust-lang.org/t/pre-rfc-forward-impls/4628/29
// named impl base
// likes `core::ops::Receiver`, can we use `Receiver`?
pub trait ShadowTrait {
    type Target: ?Sized;
}

#[fundamental]
#[repr(transparent)]
pub struct Named<N: ShadowTrait>(pub N::Target);

impl<N: ShadowTrait> Named<N> {
    pub fn new(value: N::Target) -> Self
        where N::Target: Sized,
    {
        Self(value)
    }
}

unsafe impl<N: ShadowTrait> bytemuck::TransparentWrapper<N::Target>
    for Named<N>
{
}

// impl Copy only, leave the possibility of named-impl Clone for `T: Clone + !Copy`
impl<N> Clone for Named<N>
where
    N: ShadowTrait,
    N::Target: Copy,
{
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<N> Copy for Named<N>
where
    N: ShadowTrait,
    N::Target: Copy,
{
}

pub mod wrap;
pub mod wrap_one_tag_multiple_types;

//pub mod clone;
pub mod display;
pub mod debug;

pub use wrap::Wrap;
