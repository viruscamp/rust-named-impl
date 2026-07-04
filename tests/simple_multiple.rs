use bytemuck::TransparentWrapper;
use type_tricks::ShadowTrait;
use type_tricks::Wrap;

mod share;

use share::shadow_display_impls::*;
use share::shadow_debug_impls::*;

use type_tricks::display::{ShadowDisplayProvider};
use type_tricks::debug::{ShadowDebugProvider};

pub struct SimpleMultipleTag;

impl ShadowTrait for SimpleMultipleTag {
    type Target = i32;
}

impl ShadowDisplayProvider for SimpleMultipleTag {
    type Impl = DisplayImpl1;
}

impl ShadowDebugProvider for SimpleMultipleTag {
    type Impl = DebugImpl1;
}

#[test]
fn test_simple_multiple() {
    let num = 42;

    // note: to_string() calls Display, format!("{:?}") calls Debug
    
    let a1 = Wrap::<SimpleMultipleTag>::wrap_ref(&num);
    assert_eq!(a1.to_string(), "DisplayImpl1");
    assert_eq!(format!("{a1:?}"), "DebugImpl1");
}
