use std::fmt::Display;

use bytemuck::TransparentWrapper;
use shadow_traits::Named;
use shadow_traits::ShadowTrait;
use shadow_traits::wrap1::Wrap1;

mod share;

use shadow_traits::display::DefaultDisplay;
use share::named_display_impls::*;

use shadow_traits::display::DisplayProvider1;

// use to link a serial type and impls, maybe used in rescurvice,
// when impl named Serilize
pub struct MultipleTypeTag;

impl DisplayProvider1<i32> for MultipleTypeTag {
    type Impl = DisplayImpl1;
}

impl DisplayProvider1<str> for MultipleTypeTag {
    type Impl = DefaultDisplay<str>;
}

pub struct MyType(i32, &'static str);

impl ShadowTrait for MultipleTypeTag {
    type Target = MyType;
}
// not perfect, failed with multiple level structs
impl Display for Named<MultipleTypeTag> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let i = Named::<<MultipleTypeTag as DisplayProvider1<i32>>::Impl>::wrap_ref(&self.0.0);
        let s = Named::<<MultipleTypeTag as DisplayProvider1<str>>::Impl>::wrap_ref(self.0.1);

        f.write_str(&format!("MyType(0={},1={})", &i, &s))?;

        Ok(())
    }
}

#[test]
fn test_simple_multiple() {
    let num = 42;    
    let a1 = Wrap1::<i32, MultipleTypeTag>::wrap_ref(&num);
    assert_eq!(format!("{a1}"), "DisplayImpl1");

    let s = "hello";
    let s1 = Wrap1::<str, MultipleTypeTag>::wrap_ref(s);
    assert_eq!(format!("{s1}"), "hello");

    let mytype = MyType(3, "str");
    let m1 = Wrap1::<MyType, MultipleTypeTag>::wrap(mytype);
    assert_eq!(format!("{m1}"), "MyType(0=DisplayImpl1,1=str)");
}
