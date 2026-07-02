use core::ops::{Deref, DerefMut};
use core::convert::{AsMut, AsRef};

//use core::borrow::{Borrow, BorrowMut};
//use core::convert::From;

use bytemuck::TransparentWrapper;

use crate::{NamedImplBase};

// newtype wrapper
#[fundamental]
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Wrap<NamedImpl: NamedImplBase, const ImplDeref: bool = true>
(pub NamedImpl::Target);

impl<NamedImpl: NamedImplBase, const ImplDeref: bool>
Wrap<NamedImpl, ImplDeref>
{
    pub fn new(value: NamedImpl::Target) -> Self
        where NamedImpl::Target: Sized
    {
        Wrap(value)
    }

    pub fn unwrap(self) -> NamedImpl::Target
        where NamedImpl::Target: Sized
    {
        self.0
    }

    pub fn as_ref(&self) -> &NamedImpl::Target {
        &self.0
    }

    pub fn as_mut(&mut self) -> &mut NamedImpl::Target {
        &mut self.0
    }

    pub fn to_wrap_ref(&self) -> crate::WrapRef<'_, NamedImpl> {
        crate::WrapRef::new(&self.0)
    }

    pub fn to_wrap_mut(&mut self) -> crate::WrapMut<'_, NamedImpl> {
        crate::WrapMut::new(&mut self.0)
    }
}

unsafe impl<NamedImpl: NamedImplBase, const ImplDeref: bool> TransparentWrapper<NamedImpl::Target>
for Wrap<NamedImpl, ImplDeref>
{
}

impl<NamedImpl: NamedImplBase> Deref 
    for Wrap<NamedImpl, true>
{
    type Target = NamedImpl::Target;

    fn deref(&self) -> &NamedImpl::Target {
        &self.0
    }
}

impl<NamedImpl: NamedImplBase> DerefMut 
    for Wrap<NamedImpl, true>
{
    fn deref_mut(&mut self) -> &mut NamedImpl::Target {
        &mut self.0
    }
}

impl<NamedImpl: NamedImplBase, const ImplDeref: bool>
    AsRef<NamedImpl::Target> for Wrap<NamedImpl, ImplDeref>
{
    fn as_ref(&self) -> &NamedImpl::Target {
        &self.0
    }
}

impl<NamedImpl: NamedImplBase, const ImplDeref: bool>
    AsMut<NamedImpl::Target> for Wrap<NamedImpl, ImplDeref>
{
    fn as_mut(&mut self) -> &mut NamedImpl::Target {
        &mut self.0
    }
}

// how can i prove that Wrap<NamedImpl> != NamedImpl::Target?

// confilict with impl<T> From<T> for T
// confilict with impl<T> From<NonZero<T>> for T where T: ZeroablePrimitive
// impl<NamedImpl> From<NamedImpl::Target> for Wrap<NamedImpl> 

// confilict with impl<T> Borrow<T> for T
// impl<NamedImpl> core::borrow::Borrow<NamedImpl::Target> for Wrap<NamedImpl>

// confilict with impl<T> BorrowMut<T> for T
// impl<NamedImpl> BorrowMut<NamedImpl::Target> for Wrap<NamedImpl>
