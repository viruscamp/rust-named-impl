use core::marker::PhantomData;
use core::fmt::{Display, Formatter, Result};

use type_tricks::NamedImplBase;
use type_tricks::display::NamedDisplay;

pub struct NamedDisplay1;
impl NamedImplBase for NamedDisplay1 {
    type Target = i32;
}
impl NamedDisplay for NamedDisplay1 {
    fn fmt(this: &Self::Target, f: &mut Formatter<'_>) -> Result {
        f.write_str("NamedDisplay1")
    }
}

pub struct NamedDisplayProxy<T: Display>(PhantomData<T>);
impl<T: Display> NamedImplBase for NamedDisplayProxy<T> {
    type Target = T;
}
impl<T: Display> NamedDisplay for NamedDisplayProxy<T> {
    fn fmt(this: &Self::Target, f: &mut Formatter<'_>) -> Result {
        f.write_str("Display Pre ")?;
        this.fmt(f)?;
        f.write_str(" Post")?;
        Ok(())
    }
}
