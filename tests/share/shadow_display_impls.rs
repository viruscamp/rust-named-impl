use core::marker::PhantomData;
use core::fmt::{Display, Formatter, Result};

use type_tricks::ShadowTrait;
use type_tricks::display::ShadowDisplay;

pub struct DisplayImpl1;
impl ShadowTrait<i32> for DisplayImpl1 {}
impl ShadowDisplay<i32> for DisplayImpl1 {
    fn fmt(this: &i32, f: &mut Formatter<'_>) -> Result {
        f.write_str("DisplayImpl1")
    }
}

pub struct DisplayImplProxy<T: Display + ?Sized>(PhantomData<T>);
impl<T: Display + ?Sized> ShadowTrait<T> for DisplayImplProxy<T> {}
impl<T: Display + ?Sized> ShadowDisplay<T> for DisplayImplProxy<T> {
    fn fmt(this: &T, f: &mut Formatter<'_>) -> Result {
        f.write_str("Display Pre ")?;
        this.fmt(f)?;
        f.write_str(" Post")?;
        Ok(())
    }
}
