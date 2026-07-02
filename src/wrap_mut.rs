use core::ops::{Deref, DerefMut};
use core::convert::{AsRef, AsMut};

use crate::{NamedImplBase, Wrap};

#[fundamental]
#[repr(transparent)]
pub struct WrapMut<'a, NamedImpl: NamedImplBase>(pub &'a mut NamedImpl::Target);

impl<'a, NamedImpl> WrapMut<'a, NamedImpl>
    where NamedImpl: NamedImplBase
{
    pub fn new(value: &'a mut NamedImpl::Target) -> Self {
        WrapMut(value)
    }

    pub fn unwrap(self) -> &'a mut NamedImpl::Target {
        self.0
    }

    pub fn as_ref(&self) -> &NamedImpl::Target {
        &self.0
    }

    pub fn as_mut(&mut self) -> &mut NamedImpl::Target {
        &mut self.0
    }

    pub fn to_wrap_ref(&self) -> crate::WrapRef<'_, NamedImpl> {
        crate::WrapRef::new(&*self.0)
    }
}

impl<'a, NamedImpl> Deref for WrapMut<'a, NamedImpl>
    where NamedImpl: NamedImplBase
{
    type Target = NamedImpl::Target;

    fn deref(&self) -> &NamedImpl::Target {
        &self.0
    }
}

impl<'a, NamedImpl> DerefMut for WrapMut<'a, NamedImpl>
    where NamedImpl: NamedImplBase
{
    fn deref_mut(&mut self) -> &mut NamedImpl::Target {
        &mut self.0
    }
}

impl<'a, NamedImpl> AsRef<NamedImpl::Target> for WrapMut<'a, NamedImpl>
    where NamedImpl: NamedImplBase
{
    fn as_ref(&self) -> &NamedImpl::Target {
        &self.0
    }
}

impl<'a, NamedImpl> AsMut<NamedImpl::Target> for WrapMut<'a, NamedImpl>
    where NamedImpl: NamedImplBase
{
    fn as_mut(&mut self) -> &mut NamedImpl::Target {
        &mut self.0
    }
}

impl<'a, NamedImpl> From<&'a mut Wrap<NamedImpl>> for WrapMut<'a, NamedImpl>
    where NamedImpl: NamedImplBase,
          NamedImpl::Target: Sized
{
    fn from(wrap: &'a mut Wrap<NamedImpl>) -> Self {
        WrapMut::new(&mut wrap.0)
    }
}
