use core::marker::PhantomData;
use core::fmt::{Debug, Formatter, Result};

use type_tricks::ShadowTrait;
use type_tricks::debug::ShadowDebug;

pub struct DebugImpl1;
impl ShadowTrait<i32> for DebugImpl1 {}
impl ShadowDebug<i32> for DebugImpl1 {
    fn fmt(this: &i32, f: &mut Formatter<'_>) -> Result {
        f.write_str("DebugImpl1")
    }
}

pub struct DebugImplProxy<T: Debug + ?Sized>(PhantomData<T>);
impl<T: Debug + ?Sized> ShadowTrait<T> for DebugImplProxy<T> {}
impl<T: Debug + ?Sized> ShadowDebug<T> for DebugImplProxy<T> {
    fn fmt(this: &T, f: &mut Formatter<'_>) -> Result {
        f.write_str("Debug Pre ")?;
        this.fmt(f)?;
        f.write_str(" Post")?;
        Ok(())
    }
}
