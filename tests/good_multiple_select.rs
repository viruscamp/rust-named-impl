use core::marker::PhantomData;

use bytemuck::TransparentWrapper;
use type_tricks::ShadowTrait;
use type_tricks::Wrap;

mod share;

use share::shadow_display_impls::*;
use share::shadow_debug_impls::*;

use type_tricks::display::DefaultDisplay;
use type_tricks::display::{ShadowDisplay, ShadowDisplayProvider};
use type_tricks::debug::{ShadowDebug, ShadowDebugProvider};

pub struct MultipleImplSelector<T, N1, N2>
(PhantomData<T>, PhantomData<N1>, PhantomData<N2>);

impl<T, N1, N2> ShadowTrait<T> for MultipleImplSelector<T, N1, N2>
where
    N1: ShadowDisplay<T>,
    N2: ShadowDebug<T>,
{
}

impl<T, N1, N2> ShadowDisplayProvider<T> for MultipleImplSelector<T, N1, N2>
where
    N1: ShadowDisplay<T>,
    N2: ShadowDebug<T>,
{
    type Impl = N1;
}

impl<T, N1, N2> ShadowDebugProvider<T> for MultipleImplSelector<T, N1, N2>
where
    N1: ShadowDisplay<T>,
    N2: ShadowDebug<T>,
{
    type Impl = N2;
}

#[test]
fn test_named_impl_multiple() {
    let num = 42;

    // note: to_string() calls Display, format!("{:?}") calls Debug
    
    let a1 = Wrap::<i32,MultipleImplSelector::<i32, DisplayImpl1, DebugImpl1>>::wrap_ref(&num);
    assert_eq!(a1.to_string(), "DisplayImpl1");
    assert_eq!(format!("{a1:?}"), "DebugImpl1");

    let a2 = Wrap::<i32,MultipleImplSelector::<i32, DisplayImplProxy<i32>, DebugImpl1>>::wrap_ref(&num);
    assert_eq!(a2.to_string(), "Display Pre 42 Post");
    assert_eq!(format!("{a2:?}"), "DebugImpl1");

    let a3 = Wrap::<i32,MultipleImplSelector::<i32, DefaultDisplay<i32>, DebugImplProxy<i32>>>::wrap_ref(&num);
    assert_eq!(a3.to_string(), "42");
    assert_eq!(format!("{a3:?}"), "Debug Pre 42 Post");
}
