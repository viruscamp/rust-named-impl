use core::marker::PhantomData;

use type_tricks::NamedImplBase;

use super::named_to_string::NamedToString;

pub struct NamedToString1;
impl NamedImplBase for NamedToString1 {
    type Target = i32;
}
impl NamedToString for NamedToString1 {
    fn to_string(this: &Self::Target) -> String {
        "NamedToString1".to_string()
    }
}

pub struct NamedToStringProxy<T: ToString>(PhantomData<T>);
impl<T: ToString> NamedImplBase for NamedToStringProxy<T> {
    type Target = T;
}
impl<T: ToString> NamedToString for NamedToStringProxy<T> {
    fn to_string(this: &Self::Target) -> String {
        let a = this.to_string();
        format!("Pre {a} Post")
    }
}
