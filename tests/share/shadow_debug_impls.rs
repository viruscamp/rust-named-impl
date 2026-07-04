use core::marker::PhantomData;
use core::fmt::{Debug, Formatter, Result};

use type_tricks::ShadowTrait;
use type_tricks::debug::ShadowDebug;

pub struct DebugImpl1;
impl ShadowTrait for DebugImpl1 {
    type Target = i32;
}
impl ShadowDebug for DebugImpl1 {
    fn fmt(this: &Self::Target, f: &mut Formatter<'_>) -> Result {
        f.write_str("DebugImpl1")
    }
}

pub struct DebugImplProxy<T: Debug>(PhantomData<T>);
impl<T: Debug> ShadowTrait for DebugImplProxy<T> {
    type Target = T;
}
impl<T: Debug> ShadowDebug for DebugImplProxy<T> {
    fn fmt(this: &Self::Target, f: &mut Formatter<'_>) -> Result {
        f.write_str("Debug Pre ")?;
        this.fmt(f)?;
        f.write_str(" Post")?;
        Ok(())
    }
}
