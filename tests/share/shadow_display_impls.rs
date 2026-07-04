use core::marker::PhantomData;
use core::fmt::{Display, Formatter, Result};

use type_tricks::ShadowTrait;
use type_tricks::display::ShadowDisplay;

pub struct DisplayImpl1;
impl ShadowTrait for DisplayImpl1 {
    type Target = i32;
}
impl ShadowDisplay for DisplayImpl1 {
    fn fmt(this: &Self::Target, f: &mut Formatter<'_>) -> Result {
        f.write_str("DisplayImpl1")
    }
}

pub struct DisplayImplProxy<T: Display>(PhantomData<T>);
impl<T: Display> ShadowTrait for DisplayImplProxy<T> {
    type Target = T;
}
impl<T: Display> ShadowDisplay for DisplayImplProxy<T> {
    fn fmt(this: &Self::Target, f: &mut Formatter<'_>) -> Result {
        f.write_str("Display Pre ")?;
        this.fmt(f)?;
        f.write_str(" Post")?;
        Ok(())
    }
}
