use std::marker::PhantomData;

use bytemuck::TransparentWrapper;
use type_tricks::{ShadowTrait, Wrap, is::Is};

pub trait UserSuper: Sync + Send + Copy {
    fn new() -> Self;
    fn consume(self);
}

pub trait NamedUserSuper<T : Sync + Send + Copy>: ShadowTrait<T>
{
    fn new() -> T;
    fn consume(this: T);
}

pub trait NamedUserSuperProvider<T : Sync + Send + Copy>
{
    type Impl: NamedUserSuper<T>;
}

impl< T: Sync + Send + Copy, N: NamedUserSuper<T>> NamedUserSuperProvider<T> for N
{
    type Impl = Self;
}

impl<T, NP> UserSuper for Wrap<T, NP>
where
    T: Sync + Send + Copy,
    Self: Sync + Send + Copy,
    NP: ShadowTrait<T> + NamedUserSuperProvider<T>,
{
    fn new() -> Self {
        Wrap::new(NP::Impl::new())
    }

    fn consume(self) {
        NP::Impl::consume(self.1)
    }
}

pub struct DefaultUserSuper<T: UserSuper>(PhantomData<T>);
impl<T: UserSuper> ShadowTrait<T> for DefaultUserSuper<T> {}
impl<T: UserSuper> NamedUserSuper<T> for DefaultUserSuper<T> {
    fn new() -> T {
        T::new()
    }

    fn consume(this: T) {
        T::consume(this)
    }
}

pub trait UserTrait : UserSuper {
    fn use_ref(&self);
    fn return_ref() -> &'static Self {
        let b = Box::new(Self::new());
        Box::leak(b)
    }
}

pub trait NamedUserTrait<T: Sync + Send + Copy>: ShadowTrait<T> + NamedUserSuper<T> {
    fn use_ref(this: &T);
    fn return_ref() -> &'static T {
        let b = Box::new(Self::new());
        Box::leak(b)
    }
}

pub trait NamedUserTraitProvider<T: Sync + Send + Copy>
{
    type Impl: NamedUserTrait<T>;
}

impl<N: NamedUserTrait<T>, T: Sync + Send + Copy> NamedUserTraitProvider<T> for N
{
    type Impl = Self;
}

impl<T, NP> UserTrait for Wrap<T, NP>
where
    T: Sync + Send + Copy,
    Self: Sync + Send + Copy + UserSuper,
    NP: ShadowTrait<T> + NamedUserTraitProvider<T>,
{
    fn use_ref(&self) {
        NP::Impl::use_ref(&self.1)
    }
    
    fn return_ref() -> &'static Self {
        Self::wrap_ref(NP::Impl::return_ref())
    }
}

pub struct DefaultUserTrait<T: UserTrait>(PhantomData<T>);
impl<T: UserTrait> ShadowTrait<T> for DefaultUserTrait<T> {}
impl<T: UserTrait> NamedUserSuper<T> for DefaultUserTrait<T> {
    fn new() -> T {
        <T as UserSuper>::new()
    }

    fn consume(this: T) {
        <T as UserSuper>::consume(this)
    }
}
impl<T: UserTrait> NamedUserTrait<T> for DefaultUserTrait<T> {
    fn use_ref(this: &T) {
        <T as UserTrait>::use_ref(this)
    }

    fn return_ref() -> &'static T {
        <T as UserTrait>::return_ref()
    }
}
