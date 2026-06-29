use type_tricks::Wrap;

mod share;

use share::named_to_string::*;
use share::named_to_string_impls::*;

#[test]
fn test_named_to_string() {
    let wrap1 = Wrap::<ToStringImpl<NamedToString1>>::from(123);
    assert_eq!(wrap1.to_string(), "NamedToString1");

    let wrap2 = Wrap::<ToStringImpl<NamedToStringProxy<i32>>>::from(456);
    assert_eq!(wrap2.to_string(), "Pre 456 Post");
}
