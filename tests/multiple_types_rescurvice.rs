use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

use bytemuck::TransparentWrapper;
use named_impl::Named;
use named_impl::ShadowTrait;
use named_impl::wrap_one_tag_multiple_types::WrapOneTagMultipleTypes;

mod share;

use named_impl::display::DefaultDisplay;
use share::named_display_impls::*;

use named_impl::display::DisplayProvider1;

pub struct MyType(i32, MyInnerType);

pub struct MyInnerType(i32, &'static str);

// use to link a serial type and impls, maybe used in rescurvice,
// when impl named Serilize
pub struct MultipleTypeTag;

impl DisplayProvider1<i32> for MultipleTypeTag {
    type Impl = DisplayImpl1;
}

impl DisplayProvider1<str> for MultipleTypeTag {
    type Impl = DefaultDisplay<str>;
}


pub struct MyInnerTypeWrapTag<NP>(PhantomData<NP>);
impl<NP> ShadowTrait for MyInnerTypeWrapTag<NP> {
    type Target = MyInnerType;
}
impl<NP> Display for Named<MyInnerTypeWrapTag<NP>>
where 
    NP: DisplayProvider1<i32>,
    Named<<NP as DisplayProvider1<i32>>::Impl>: Display,
    NP: DisplayProvider1<str>,
    Named<<NP as DisplayProvider1<str>>::Impl>: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let i = Named::<<NP as DisplayProvider1<i32>>::Impl>::wrap_ref(&self.0.0);
        let s = Named::<<NP as DisplayProvider1<str>>::Impl>::wrap_ref(self.0.1);

        f.write_str(&format!("MyInnerType(0={},1={})", &i, &s))?;

        Ok(())
    }
}
impl DisplayProvider1<MyInnerType> for MultipleTypeTag {
    type Impl = MyInnerTypeWrapTag<MultipleTypeTag>;
}


pub struct MyTypeWrapTag<NP>(PhantomData<NP>);
impl<NP> ShadowTrait for MyTypeWrapTag<NP> {
    type Target = MyType;
}
impl<NP> Display for Named<MyTypeWrapTag<NP>>
where 
    NP: DisplayProvider1<i32>,
    Named<<NP as DisplayProvider1<i32>>::Impl>: Display,
    NP: DisplayProvider1<MyInnerType>,
    Named<<NP as DisplayProvider1<MyInnerType>>::Impl>: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let i = Named::<<NP as DisplayProvider1<i32>>::Impl>::wrap_ref(&self.0.0);
        let s = Named::<<NP as DisplayProvider1<MyInnerType>>::Impl>::wrap_ref(&self.0.1);

        f.write_str(&format!("MyType(0={},1={})", &i, &s))?;

        Ok(())
    }
}
impl DisplayProvider1<MyType> for MultipleTypeTag {
    type Impl = MyTypeWrapTag<MultipleTypeTag>;
}

#[test]
fn test_multiple_types_rescurvice() {
    let num = 42;    
    let a1 = WrapOneTagMultipleTypes::<i32, MultipleTypeTag>::wrap_ref(&num);
    assert_eq!(format!("{a1}"), "DisplayImpl1");

    let s = "hello";
    let s1 = WrapOneTagMultipleTypes::<str, MultipleTypeTag>::wrap_ref(s);
    assert_eq!(format!("{s1}"), "hello");

    let myinnertype = MyInnerType(num, s);
    let mi1 = WrapOneTagMultipleTypes::<MyInnerType, MultipleTypeTag>::wrap(myinnertype);
    assert_eq!(format!("{mi1}"), "MyInnerType(0=DisplayImpl1,1=hello)");

    let mytype = MyType(num, MyInnerType(num, s));
    let m1 = WrapOneTagMultipleTypes::<MyType, MultipleTypeTag>::wrap(mytype);
    assert_eq!(format!("{m1}"), "MyType(0=DisplayImpl1,1=MyInnerType(0=DisplayImpl1,1=hello))");
}
