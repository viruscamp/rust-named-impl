use type_tricks::is_copy::{IsCopy, NotCopy};
use type_tricks::{ W, True, False };

struct Test1<T>(T) where W<T>: IsCopy<Answer = False>;

struct Test2<T>(T) where W<T>: NotCopy;

#[test]
fn should_pass_if_is_not_copy() {
    struct NonCopy;
    let _ = Test1(NonCopy);
    let _ = Test2(NonCopy);
}
