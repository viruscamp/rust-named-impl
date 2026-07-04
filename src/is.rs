pub trait Is {
    type Type: ?Sized;

    fn to_right(left: Self) -> Self::Type where Self: Sized;
    fn to_left(right: Self::Type) -> Self where Self: Sized;
    fn to_ref_right(left: &Self) -> &Self::Type;
    fn to_ref_left(right: &Self::Type) -> &Self;
    fn to_mut_right(left: &mut Self) -> &mut Self::Type;
    fn to_mut_left(right: &mut Self::Type) -> &mut Self;
}

impl<T: ?Sized> Is for T {
    type Type = T;

    fn to_right(left: Self) -> Self::Type where Self: Sized {
        left
    }

    fn to_left(right: Self::Type) -> Self where Self: Sized {
        right
    }

    fn to_ref_right(left: &Self) -> &Self::Type {
        left
    }

    fn to_ref_left(right: &Self::Type) -> &Self {
        right
    }

    fn to_mut_right(left: &mut Self) -> &mut Self::Type {
        left
    }

    fn to_mut_left(right: &mut Self::Type) -> &mut Self {
        right
    }
}

// https://docs.rs/higher-kinded-types/latest/src/higher_kinded_types/advanced/type_eq.rs.html#99-100

#[cfg(feature = "alloc")]
pub fn to_box_left<T>(left: Box<T>) -> Box<<T as Is>::Type>
{
    left
}

#[cfg(feature = "alloc")]
pub fn to_box_right<T>(right: Box<<T as Is>::Type>) -> Box<T> {
    right
}
