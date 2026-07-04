pub trait Is {
    type Type: ?Sized;

    fn to_right(self) -> Self::Type where Self: Sized;
    fn to_left(t: Self::Type) -> Self where Self: Sized;
    fn to_ref_right(&self) -> &Self::Type;
    fn to_ref_left(t: &Self::Type) -> &Self;
    fn to_mut_right(&mut self) -> &mut Self::Type;
    fn to_mut_left(t: &mut Self::Type) -> &mut Self;
}

impl<T: ?Sized> Is for T {
    type Type = T;

    fn to_right(self) -> Self::Type where Self: Sized {
        self
    }

    fn to_left(t: Self::Type) -> Self where Self: Sized {
        t
    }

    fn to_ref_right(&self) -> &Self::Type {
        self
    }

    fn to_ref_left(t: &Self::Type) -> &Self {
        t
    }

    fn to_mut_right(&mut self) -> &mut Self::Type {
        self
    }

    fn to_mut_left(t: &mut Self::Type) -> &mut Self {
        t
    }
}

// https://docs.rs/higher-kinded-types/latest/src/higher_kinded_types/advanced/type_eq.rs.html#99-100

#[cfg(feature = "alloc")]
pub fn into_box_val<T>(it: Box<T>) -> Box<<T as Is>::Type>
{
    it
}

#[cfg(feature = "alloc")]
pub fn from_box_val<T>(x: Box<<T as Is>::Type>) -> Box<T> {
    x
}
