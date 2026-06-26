use std::ops::Not;

use crate::{W, True, False};

pub trait IsCopy {
    type const Impled: bool;
    type Answer;
}

impl<T> IsCopy for W<T> {
    default type const Impled: bool = false;
    default type Answer = False;
}

impl<T: Copy> IsCopy for W<T> {
    type const Impled: bool = true;
    type Answer = True;
}

pub trait NotCopy : IsCopy {}
impl<T: ?Sized + IsCopy<Answer = False>> NotCopy for T {}
