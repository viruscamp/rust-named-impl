pub trait Is {
    type Type: ?Sized;

    fn into_val<X: Sized>(x: X) -> <X as Is>::Type;
    fn from_val<X: Sized>(t: <X as Is>::Type) -> X;
    fn into_ref(&self) -> &Self::Type;
    fn into_mut_ref(&mut self) -> &mut Self::Type;
    fn from_ref(t: &Self::Type) -> &Self;
    fn from_mut_ref(t: &mut Self::Type) -> &mut Self;
}

impl<T: ?Sized> Is for T {
    type Type = T;

    fn into_val<X: Sized>(x: X) -> <X as Is>::Type {
        x
    }

    fn from_val<X: Sized>(t: <X as Is>::Type) -> X {
        t
    }

    fn into_ref(&self) -> &Self::Type {
        self
    }

    fn into_mut_ref(&mut self) -> &mut Self::Type {
        self
    }

    fn from_ref(t: &Self::Type) -> &Self {
        t
    }

    fn from_mut_ref(t: &mut Self::Type) -> &mut Self {
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
