use core::marker::PhantomData;
use core::fmt::{Debug, Formatter, Result};

use bytemuck::TransparentWrapper;
use type_tricks::wrap::Wrap;
use type_tricks::ShadowTrait;
use type_tricks::debug::ShadowDebug;
use type_tricks::is::Is;

mod share;

use share::shadow_to_string::*;
use share::shadow_to_string_impls::*;
use share::shadow_debug_impls::*;


pub struct MultipleImplSelector<T, N1, N2>
(PhantomData<T>, PhantomData<N1>, PhantomData<N2>);

impl<T, TS, D> ShadowTrait for MultipleImplSelector<T, TS, D>
where
    TS: ShadowToString,
    TS::Target: Is<Type = T>,
    D: ShadowDebug,
    D::Target: Is<Type = T>,
{
    type Target = T;
}

impl<T, TS, D> ToString for Wrap<MultipleImplSelector<T, TS, D>>
where
    TS: ShadowToString,
    TS::Target: Is<Type = T>,
    D: ShadowDebug,
    D::Target: Is<Type = T>,
{
    fn to_string(&self) -> String {
        let a: &T = &self.0;
        let b: &TS::Target = Is::to_ref_left(a);
        TS::to_string(b)
    }
}

impl<T, TS, D> Debug for Wrap<MultipleImplSelector<T, TS, D>>
where
    TS: ShadowToString,
    TS::Target: Is<Type = T>,
    D: ShadowDebug,
    D::Target: Is<Type = T>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let a: &T = &self.0;
        let b: &D::Target = Is::to_ref_left(a);
        D::fmt(b, f)
    }
}

#[test]
fn test_named_impl_multiple() {
    let num = 42;

    let a1 = Wrap::<MultipleImplSelector::<i32, ToStringImpl1, DebugImpl1>>::wrap_ref(&num);
    assert_eq!(a1.to_string(), "ToStringImpl1");
    assert_eq!(format!("{a1:?}"), "DebugImpl1");

    let a2 = Wrap::<MultipleImplSelector::<i32, ToStringImplProxy<i32>, DebugImpl1>>::wrap_ref(&num);
    assert_eq!(a2.to_string(), "Pre 42 Post");
    assert_eq!(format!("{a2:?}"), "DebugImpl1");

    let a3 = Wrap::<MultipleImplSelector::<i32, DefaultToString<i32>, DebugImplProxy<i32>>>::wrap_ref(&num);
    assert_eq!(a3.to_string(), "42");
    assert_eq!(format!("{a3:?}"), "Debug Pre 42 Post");
}
