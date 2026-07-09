use core::convert::{AsMut, AsRef};
use core::ops::{Deref, DerefMut};

//use core::borrow::{Borrow, BorrowMut};
//use core::convert::From;

use bytemuck::TransparentWrapper;

use crate::ShadowTrait;

// newtype wrapper
#[fundamental]
#[repr(transparent)]
pub struct Wrap<NP: ShadowTrait, const IMPL_DEREF: bool = true>(pub NP::Target);

impl<NP, const IMPL_DEREF: bool> Clone for Wrap<NP, IMPL_DEREF>
where
    NP: ShadowTrait,
    NP::Target: Copy,
{
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<NP, const IMPL_DEREF: bool> Copy for Wrap<NP, IMPL_DEREF>
where
    NP: ShadowTrait,
    NP::Target: Copy,
{
}

impl<NP: ShadowTrait, const IMPL_DEREF: bool> Wrap<NP, IMPL_DEREF> {
    pub fn new(value: NP::Target) -> Self
        where NP::Target: Sized,
    {
        Self(value)
    }
}

unsafe impl<NP: ShadowTrait, const IMPL_DEREF: bool> TransparentWrapper<NP::Target>
    for Wrap<NP, IMPL_DEREF>
{
}

impl<NP: ShadowTrait> Deref for Wrap<NP, true> {
    type Target = NP::Target;

    fn deref(&self) -> &NP::Target {
        &self.0
    }
}

impl<NP: ShadowTrait> DerefMut for Wrap<NP, true> {
    fn deref_mut(&mut self) -> &mut NP::Target {
        &mut self.0
    }
}

impl<NP: ShadowTrait> AsRef<NP::Target> for Wrap<NP, true> {
    fn as_ref(&self) -> &NP::Target {
        &self.0
    }
}

impl<NP: ShadowTrait> AsMut<NP::Target> for Wrap<NP, true> {
    fn as_mut(&mut self) -> &mut NP::Target {
        &mut self.0
    }
}

// how can i prove that Wrap<NP> != NP::Target?

// confilict with impl<T> From<T> for T
// confilict with impl<T> From<NonZero<T>> for T where T: ZeroablePrimitive
// impl<NP> From<NP::Target> for Wrap<NP>

// confilict with impl<T> Borrow<T> for T
// impl<NP> Borrow<NP::Target> for Wrap<NP>

// confilict with impl<T> BorrowMut<T> for T
// impl<NP> BorrowMut<NP::Target> for Wrap<NP>
