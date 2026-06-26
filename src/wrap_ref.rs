use core::marker::PhantomData;
use core::ops::Deref;
use core::convert::AsRef;

use crate::{NamedImplBase, Wrap, WrapMut};

#[fundamental]
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct WrapRef<'a, NamedImpl>
    where NamedImpl: NamedImplBase
{
    pub value: &'a NamedImpl::Target,
    pub phantom: PhantomData<NamedImpl>,
}

impl<'a, NamedImpl> !NamedImplBase for WrapRef<'a, NamedImpl> {}

impl<'a, NamedImpl> WrapRef<'a, NamedImpl>
    where NamedImpl: NamedImplBase
{
    pub fn from(value: &'a NamedImpl::Target) -> Self {
        WrapRef {
            value,
            phantom: PhantomData,
        }
    }

    pub fn into(self) -> &'a NamedImpl::Target {
        self.value
    }
}

impl<'a, NamedImpl> Deref for WrapRef<'a, NamedImpl>
    where NamedImpl: NamedImplBase
{
    type Target = NamedImpl::Target;

    fn deref(&self) -> &NamedImpl::Target {
        &self.value
    }
}

impl<'a, NamedImpl> AsRef<NamedImpl::Target> for WrapRef<'a, NamedImpl>
    where NamedImpl: NamedImplBase
{
    fn as_ref(&self) -> &NamedImpl::Target {
        &self.value
    }
}

impl<'a, NamedImpl> From<&'a Wrap<NamedImpl>> for WrapRef<'a, NamedImpl>
    where NamedImpl: NamedImplBase,
          NamedImpl::Target: Sized
{
    fn from(wrap: &'a Wrap<NamedImpl>) -> Self {
        WrapRef::from(&wrap.value)
    }
}

impl<'a, NamedImpl> From<&'a WrapMut<'a, NamedImpl>> for WrapRef<'a, NamedImpl>
    where NamedImpl: NamedImplBase
{
    fn from(wrap: &'a WrapMut<'a, NamedImpl>) -> Self {
        WrapRef::from(wrap.value)
    }
}
