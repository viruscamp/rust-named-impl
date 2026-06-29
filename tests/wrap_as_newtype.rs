use core::fmt::{Debug, Formatter, Result};

use type_tricks::{NamedImplBase, Wrap};

mod share;
use share::named_debug_impls::NamedDebugProxy;

use crate::share::named_debug::NamedDebug;

struct WrapI32Tag;
impl NamedImplBase for WrapI32Tag {
    type Target = i32;
}
type WrapI32 = Wrap<WrapI32Tag>;

impl ToString for WrapI32 {
    fn to_string(&self) -> String {
        format!("WrapI32({})", self.value)
    }
}

impl Debug for WrapI32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        NamedDebugProxy::fmt(&self.value, f)
    }
}

#[test]
fn test_wrap_as_newtype() {
    let wrap = WrapI32::from(42);
    assert_eq!(wrap.to_string(), "WrapI32(42)");
    assert_eq!(format!("{wrap:?}"), "Debug Pre 42 Post");
}
