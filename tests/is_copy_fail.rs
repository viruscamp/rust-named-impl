use type_tricks::is_copy::IsCopy;
use type_tricks::{ W, True, False };

struct Test<T>(T) where W<T>: IsCopy<Answer = False>;

#[test]
fn should_fail_if_is_copy() {
    let _ = Test(42);
}