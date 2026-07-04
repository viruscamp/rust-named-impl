use core::marker::PhantomData;

use type_tricks::ShadowTrait;

use super::shadow_to_string::ShadowToString;

pub struct ToStringImpl1;
impl ShadowTrait for ToStringImpl1 {
    type Target = i32;
}
impl ShadowToString for ToStringImpl1 {
    fn to_string(this: &Self::Target) -> String {
        "ToStringImpl1".to_string()
    }
}

pub struct ToStringImplProxy<T: ToString>(PhantomData<T>);
impl<T: ToString> ShadowTrait for ToStringImplProxy<T> {
    type Target = T;
}
impl<T: ToString> ShadowToString for ToStringImplProxy<T> {
    fn to_string(this: &Self::Target) -> String {
        let a = this.to_string();
        format!("Pre {a} Post")
    }
}
