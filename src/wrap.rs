use core::ops::{Deref, DerefMut};
use core::convert::{AsMut, AsRef};
use core::marker::PhantomData;

//use core::borrow::{Borrow, BorrowMut};
//use core::convert::From;

use bytemuck::TransparentWrapper;

use crate::ShadowTrait;

// newtype wrapper
//#[fundamental]
#[repr(transparent)]
pub struct Wrap<T: ?Sized, N: ShadowTrait<T>, const ImplDeref: bool = true>
(PhantomData<N>, pub T);

impl<T: Copy + ?Sized, N: ShadowTrait<T>, const ImplDeref: bool>
Clone for Wrap<T, N, ImplDeref> {
    fn clone(&self) -> Self {
        Wrap(PhantomData, self.1)
    }
}

impl<T: Copy + ?Sized, N: ShadowTrait<T>, const ImplDeref: bool>
Copy for Wrap<T, N, ImplDeref> {}

impl<T: ?Sized, N: ShadowTrait<T>, const ImplDeref: bool>
Wrap<T, N, ImplDeref>
{
    pub fn new(value: T) -> Self
        where T: Sized
    {
        Wrap(PhantomData, value)
    }

    pub fn unwrap(self) -> T
        where T: Sized
    {
        self.1
    }

    pub fn as_ref(&self) -> &T {
        &self.1
    }

    pub fn as_mut(&mut self) -> &mut T {
        &mut self.1
    }
}

unsafe impl<T: ?Sized, N: ShadowTrait<T>, const ImplDeref: bool>
TransparentWrapper<T>
for Wrap<T, N, ImplDeref>
{
}

impl<T: ?Sized, N: ShadowTrait<T>> Deref 
    for Wrap<T, N, true>
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.1
    }
}

impl<T: ?Sized, N: ShadowTrait<T>> DerefMut 
    for Wrap<T, N, true>
{
    fn deref_mut(&mut self) -> &mut T {
        &mut self.1
    }
}

impl<T: ?Sized, N: ShadowTrait<T>>
    AsRef<T> for Wrap<T, N, true>
{
    fn as_ref(&self) -> &T {
        &self.1
    }
}

impl<T: ?Sized, N: ShadowTrait<T>>
    AsMut<T> for Wrap<T, N, true>
{
    fn as_mut(&mut self) -> &mut T {
        &mut self.1
    }
}

// how can i prove that Wrap<N> != N::Target?

// confilict with impl<T> From<T> for T
// confilict with impl<T> From<NonZero<T>> for T where T: ZeroablePrimitive
// impl<N> From<N::Target> for Wrap<N> 

// confilict with impl<T> Borrow<T> for T
// impl<N> core::borrow::Borrow<N::Target> for Wrap<N>

// confilict with impl<T> BorrowMut<T> for T
// impl<N> BorrowMut<N::Target> for Wrap<N>
