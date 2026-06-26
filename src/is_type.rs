use crate::{W, True, False};

pub trait IsType<T: ?Sized> {
    type Answer;
}

impl<U: ?Sized, T: ?Sized> IsType<U> for W<T> {
    default type Answer = False;
}

impl<T: ?Sized> IsType<T> for W<T> {
    type Answer = True;
}
