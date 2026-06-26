use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use core::convert::{AsMut, AsRef};

//use core::borrow::{Borrow, BorrowMut};
//use core::convert::From;

use crate::NamedImplBase;

// newtype wrapper
#[fundamental]
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Wrap<NamedImpl>
where
    NamedImpl: NamedImplBase,
    NamedImpl::Target: Sized,
{
    pub value: NamedImpl::Target,
    pub phantom: PhantomData<NamedImpl>,
}

impl<NamedImpl> !NamedImplBase for Wrap<NamedImpl> {}

impl<NamedImpl> Wrap<NamedImpl>
where
    NamedImpl: NamedImplBase,
    NamedImpl::Target: Sized,
{
    pub fn from(value: NamedImpl::Target) -> Self {
        Wrap {
            value,
            phantom: PhantomData,
        }
    }

    pub fn into(self) -> NamedImpl::Target {
        self.value
    }
}

impl<NamedImpl> Deref for Wrap<NamedImpl>
where
    NamedImpl: NamedImplBase,
    NamedImpl::Target: Sized,
{
    type Target = NamedImpl::Target;

    fn deref(&self) -> &NamedImpl::Target {
        &self.value
    }
}

impl<NamedImpl> DerefMut for Wrap<NamedImpl>
where
    NamedImpl: NamedImplBase,
    NamedImpl::Target: Sized,
{
    fn deref_mut(&mut self) -> &mut NamedImpl::Target {
        &mut self.value
    }
}

impl<NamedImpl> AsRef<NamedImpl::Target> for Wrap<NamedImpl>
where
    NamedImpl: NamedImplBase,
    NamedImpl::Target: Sized,
{
    fn as_ref(&self) -> &NamedImpl::Target {
        &self.value
    }
}

impl<NamedImpl> AsMut<NamedImpl::Target> for Wrap<NamedImpl>
where
    NamedImpl: NamedImplBase,
    NamedImpl::Target: Sized,
{
    fn as_mut(&mut self) -> &mut NamedImpl::Target {
        &mut self.value
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
