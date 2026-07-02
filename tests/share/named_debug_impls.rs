use core::marker::PhantomData;
use core::fmt::{Debug, Formatter, Result};

use type_tricks::NamedImplBase;
use type_tricks::debug::NamedDebug;

pub struct NamedDebug1;
impl NamedImplBase for NamedDebug1 {
    type Target = i32;
}
impl NamedDebug for NamedDebug1 {
    fn fmt(this: &Self::Target, f: &mut Formatter<'_>) -> Result {
        f.write_str("NamedDebug1")
    }
}

pub struct NamedDebugProxy<T: Debug>(PhantomData<T>);
impl<T: Debug> NamedImplBase for NamedDebugProxy<T> {
    type Target = T;
}
impl<T: Debug> NamedDebug for NamedDebugProxy<T> {
    fn fmt(this: &Self::Target, f: &mut Formatter<'_>) -> Result {
        f.write_str("Debug Pre ")?;
        this.fmt(f)?;
        f.write_str(" Post")?;
        Ok(())
    }
}
