use core::marker::PhantomData;

use type_tricks::ShadowTrait;

use super::shadow_to_string::ShadowToString;

pub struct ToStringImpl1;
impl ShadowTrait<i32> for ToStringImpl1 {}
impl ShadowToString<i32> for ToStringImpl1 {
    fn to_string(this: &i32) -> String {
        "ToStringImpl1".to_string()
    }
}

pub struct ToStringImplProxy<T: ToString + ?Sized>(PhantomData<T>);
impl<T: ToString + ?Sized> ShadowTrait<T> for ToStringImplProxy<T> {}
impl<T: ToString + ?Sized> ShadowToString<T> for ToStringImplProxy<T> {
    fn to_string(this: &T) -> String {
        let a = this.to_string();
        format!("Pre {a} Post")
    }
}
