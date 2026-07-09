use core::convert::{AsMut, AsRef};
use core::ops::{Deref, DerefMut};

//use core::borrow::{Borrow, BorrowMut};
//use core::convert::From;

use bytemuck::TransparentWrapper;

use crate::ShadowTrait;

// newtype wrapper
#[fundamental]
#[repr(transparent)]
pub struct Wrap<NamedImpl: ShadowTrait, const IMPL_DEREF: bool = true>(pub NamedImpl::Target);

impl<NamedImpl, const IMPL_DEREF: bool> Clone for Wrap<NamedImpl, IMPL_DEREF>
where
    NamedImpl: ShadowTrait,
    NamedImpl::Target: Copy,
{
    fn clone(&self) -> Self {
        Wrap(self.0)
    }
}

impl<NamedImpl, const IMPL_DEREF: bool> Copy for Wrap<NamedImpl, IMPL_DEREF>
where
    NamedImpl: ShadowTrait,
    NamedImpl::Target: Copy,
{
}

impl<NamedImpl: ShadowTrait, const IMPL_DEREF: bool> Wrap<NamedImpl, IMPL_DEREF> {
    pub fn new(value: NamedImpl::Target) -> Self
        where NamedImpl::Target: Sized,
    {
        Wrap(value)
    }

    pub fn unwrap(self) -> NamedImpl::Target
        where NamedImpl::Target: Sized,
    {
        self.0
    }

    pub fn as_ref(&self) -> &NamedImpl::Target {
        &self.0
    }

    pub fn as_mut(&mut self) -> &mut NamedImpl::Target {
        &mut self.0
    }
}

unsafe impl<NamedImpl: ShadowTrait, const IMPL_DEREF: bool> TransparentWrapper<NamedImpl::Target>
    for Wrap<NamedImpl, IMPL_DEREF>
{
}

impl<NamedImpl: ShadowTrait> Deref for Wrap<NamedImpl, true> {
    type Target = NamedImpl::Target;

    fn deref(&self) -> &NamedImpl::Target {
        &self.0
    }
}

impl<NamedImpl: ShadowTrait> DerefMut for Wrap<NamedImpl, true> {
    fn deref_mut(&mut self) -> &mut NamedImpl::Target {
        &mut self.0
    }
}

impl<NamedImpl: ShadowTrait> AsRef<NamedImpl::Target> for Wrap<NamedImpl, true> {
    fn as_ref(&self) -> &NamedImpl::Target {
        &self.0
    }
}

impl<NamedImpl: ShadowTrait> AsMut<NamedImpl::Target> for Wrap<NamedImpl, true> {
    fn as_mut(&mut self) -> &mut NamedImpl::Target {
        &mut self.0
    }
}

// how can i prove that Wrap<NamedImpl> != NamedImpl::Target?

// confilict with impl<T> From<T> for T
// confilict with impl<T> From<NonZero<T>> for T where T: ZeroablePrimitive
// impl<NamedImpl> From<NamedImpl::Target> for Wrap<NamedImpl>

// confilict with impl<T> Borrow<T> for T
// impl<NamedImpl> Borrow<NamedImpl::Target> for Wrap<NamedImpl>

// confilict with impl<T> BorrowMut<T> for T
// impl<NamedImpl> BorrowMut<NamedImpl::Target> for Wrap<NamedImpl>
