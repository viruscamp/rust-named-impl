use core::marker::PhantomData;

//use core::borrow::{Borrow, BorrowMut};
//use core::convert::From;

use bytemuck::TransparentWrapper;

/// It tries to make the Tag `N` to hold named-impls for different types.  
/// use to link a serial type and impls, possibly in a recursive manner,  
/// for example when implementing named serialization traits.  
/// However, it lacks the ability to impl external trait on Wrap of local Tag.  
/// As `#[fundamental]` on `Wrap<T, N>` with more than one generic paraments is invalid.  
#[fundamental]
#[repr(transparent)]
pub struct Wrap1<T: ?Sized, N>(PhantomData<N>, pub T);

impl<T: Copy + ?Sized, N> Clone for Wrap1<T, N> {
    fn clone(&self) -> Self {
        Self(PhantomData, self.1)
    }
}

impl<T: Copy + ?Sized, N> Copy for Wrap1<T, N> {}

unsafe impl<T: ?Sized, N> TransparentWrapper<T> for Wrap1<T, N> {}
