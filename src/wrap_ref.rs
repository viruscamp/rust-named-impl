use core::ops::Deref;
use core::convert::AsRef;

use crate::{NamedImplBase, Wrap, WrapMut};

#[fundamental]
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct WrapRef<'a, NamedImpl: NamedImplBase>(pub &'a NamedImpl::Target);

impl<'a, NamedImpl> WrapRef<'a, NamedImpl>
    where NamedImpl: NamedImplBase
{
    pub fn new(value: &'a NamedImpl::Target) -> Self {
        WrapRef(value)
    }

    pub fn unwrap(self) -> &'a NamedImpl::Target {
        self.0
    }

    pub fn as_ref(&self) -> &NamedImpl::Target {
        self.0
    }
}

impl<'a, NamedImpl> Deref for WrapRef<'a, NamedImpl>
    where NamedImpl: NamedImplBase
{
    type Target = NamedImpl::Target;

    fn deref(&self) -> &NamedImpl::Target {
        &self.0
    }
}

impl<'a, NamedImpl> AsRef<NamedImpl::Target> for WrapRef<'a, NamedImpl>
    where NamedImpl: NamedImplBase
{
    fn as_ref(&self) -> &NamedImpl::Target {
        &self.0
    }
}

impl<'a, NamedImpl> From<&'a Wrap<NamedImpl>> for WrapRef<'a, NamedImpl>
    where NamedImpl: NamedImplBase,
          NamedImpl::Target: Sized
{
    fn from(wrap: &'a Wrap<NamedImpl>) -> Self {
        WrapRef::new(&wrap.0)
    }
}

impl<'a, NamedImpl> From<&'a WrapMut<'a, NamedImpl>> for WrapRef<'a, NamedImpl>
    where NamedImpl: NamedImplBase
{
    fn from(wrap: &'a WrapMut<'a, NamedImpl>) -> Self {
        WrapRef::new(wrap.0)
    }
}
