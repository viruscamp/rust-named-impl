use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use core::convert::{AsRef, AsMut};

use crate::{NamedImplBase, Wrap};

#[fundamental]
#[repr(transparent)]
pub struct WrapMut<'a, NamedImpl>
    where NamedImpl: NamedImplBase
{
    pub value: &'a mut NamedImpl::Target,
    pub phantom: PhantomData<NamedImpl>,
}

impl<'a, NamedImpl> !NamedImplBase for WrapMut<'a, NamedImpl> {}

impl<'a, NamedImpl> WrapMut<'a, NamedImpl>
    where NamedImpl: NamedImplBase
{
    pub fn from(value: &'a mut NamedImpl::Target) -> Self {
        WrapMut {
            value,
            phantom: PhantomData,
        }
    }

    pub fn into(self) -> &'a mut NamedImpl::Target {
        self.value
    }
}

impl<'a, NamedImpl> Deref for WrapMut<'a, NamedImpl>
    where NamedImpl: NamedImplBase
{
    type Target = NamedImpl::Target;

    fn deref(&self) -> &NamedImpl::Target {
        &self.value
    }
}

impl<'a, NamedImpl> DerefMut for WrapMut<'a, NamedImpl>
    where NamedImpl: NamedImplBase
{
    fn deref_mut(&mut self) -> &mut NamedImpl::Target {
        &mut self.value
    }
}

impl<'a, NamedImpl> AsRef<NamedImpl::Target> for WrapMut<'a, NamedImpl>
    where NamedImpl: NamedImplBase
{
    fn as_ref(&self) -> &NamedImpl::Target {
        &self.value
    }
}

impl<'a, NamedImpl> AsMut<NamedImpl::Target> for WrapMut<'a, NamedImpl>
    where NamedImpl: NamedImplBase
{
    fn as_mut(&mut self) -> &mut NamedImpl::Target {
        &mut self.value
    }
}

impl<'a, NamedImpl> From<&'a mut Wrap<NamedImpl>> for WrapMut<'a, NamedImpl>
    where NamedImpl: NamedImplBase,
          NamedImpl::Target: Sized
{
    fn from(wrap: &'a mut Wrap<NamedImpl>) -> Self {
        WrapMut::from(&mut wrap.value)
    }
}
