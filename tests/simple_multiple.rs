use bytemuck::TransparentWrapper;
use type_tricks::NamedImplBase;
use type_tricks::Wrap;

mod share;

use share::named_display_impls::*;
use share::named_debug_impls::*;

use type_tricks::display::{NamedDisplayProvider};
use type_tricks::debug::{NamedDebugProvider};

pub struct SimpleMultipleTag;

impl NamedImplBase for SimpleMultipleTag {
    type Target = i32;
}

impl NamedDisplayProvider for SimpleMultipleTag {
    type Impl = NamedDisplay1;
}

impl NamedDebugProvider for SimpleMultipleTag {
    type Impl = NamedDebug1;
}

#[test]
fn test_simple_multiple() {
    let num = 42;

    // note: to_string() calls Display, format!("{:?}") calls Debug
    
    let a1 = Wrap::<SimpleMultipleTag>::wrap_ref(&num);
    assert_eq!(a1.to_string(), "NamedDisplay1");
    assert_eq!(format!("{a1:?}"), "NamedDebug1");
}
