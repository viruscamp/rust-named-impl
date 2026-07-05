# An experiment lib to simulate the named impl draft

There are lots draft or post try to relax the orphan rule. One of them is my
[Draft: Named impl with Implementation Selection Variant](https://internals.rust-lang.org/t/named-impl-with-implementation-selection-variant/24374). After hard working, I have wrote this lib to simulate most features in current Rust Edition.  

This lib is based on a concept called 'shadow trait' and a shared wrap type.

## Concept of 'shadow trait'
If there is a trait `Trait1`, then we define a new trait call `ShadowTrait1` which is modified from `Trait1` using the rules below:
1. Visibility remains unchanged.
2. Add a new supertrait `ShadowTrait`.
3. Retain all generic parameters, with their trait bounds unchanged.
4. Associated items such as `type Item;` and `const Count: u64;` remain unchanged.
5. For all associated function definitions, including their parameters, generics, and bounds:
    - Replace the `Self` type with `<Self as ShadowTrait>::Target`.
    - Replace the receiver `self` with this: `<Self as ShadowTrait>::Target`.
    - Replace the receiver `&self` with this: `&<Self as ShadowTrait>::Target`.
    - Replace the receiver `&mut self` with this: `&mut <Self as ShadowTrait>::Target`.
    - Replace a custom receiver like `self: Box<Self>` with `this: Box<<Self as ShadowTrait>::Target>`.
6. List all supertraits. Marker traits such as Copy, Send, Sync, Unpin, and ?Sized should be moved to constraints on Target.
7. Other supertraits should be changed to their corresponding shadow traits.  
    Example:
    ```rust
    trait Trait1: Display + Send {
        fn new() -> Self;
        fn consume(self);
        fn use_ref(&self);
        fn return_ref() -> &'static Self;
    }

    trait ShadowTrait1: ShadowTrait + ShadowDisplay 
        where Self::Target: Send 
    {
        fn new() -> Self::Target;
        fn consume(this: Self::Target);
        fn use_ref(this: &Self::Target);
        fn return_ref() -> &'static Self::Target;
    }
    ```

# Explain the lib and usage

## 1. base trait, should be included in rust core lib
provides: `ShadowTrait`  
```rust
pub trait ShadowTrait {
    type Target: ?Sized;
}
```

## 2. wrap, should be included in rust core lib
depends: `ShadowTrait`  
provides: `Wrap`  
```rust
#[fundamental]
#[repr(transparent)]
pub struct Wrap<N: ShadowTrait>(pub N::Target);
```

## 3. shadow traits provider
depends: `ShadowTrait`
  * The 1st choice is to be provided by the language as `static Trait`
  * The 2nd choice is to be defined along with the orignal Trait
  * The 3rd choice is in external crates with a determined name. 'shadow-traits-core' 'shadow-traits-std' 'shadow-traits-serde'
```rust
// required
pub trait ShadowDisplay: ShadowTrait {
    fn fmt(this: &Self::Target, f: &mut Formatter<'_>) -> Result;
}

// optional, external crates cannot have this
pub trait ShadowDisplayProvider: ShadowTrait
    where <Self::Impl as ShadowTrait>::Target: Is<Type = Self::Target>
{
    type Impl: ShadowDisplay;
}
impl<N: ShadowDisplay> ShadowDisplayProvider for N {
    type Impl = Self;
}
impl<NP: ShadowDisplayProvider> Display for crate::Wrap<NP> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        NP::Impl::fmt(Is::to_ref_left(&self.0), f)
    }
}

// optinal, a default impl
pub struct DefaultDisplay<T: Display>(PhantomData<T>);
impl<T: Display> ShadowTrait for DefaultDisplay<T> {
    type Target = T;
}
impl<T: Display> ShadowDisplay for DefaultDisplay<T> {
    fn fmt(this: &Self::Target, f: &mut Formatter<'_>) -> Result {
        <T as Display>::fmt(this, f)
    }
}
```

## 4. named impls provider, should be an external lib
depends `ShadowTrait`, shadow traits provider
```rust
pub struct DisplayImpl1;
impl ShadowTrait for DisplayImpl1 {
    type Target = i32;
}
impl ShadowDisplay for DisplayImpl1 {
    fn fmt(this: &Self::Target, f: &mut Formatter<'_>) -> Result {
        f.write_str("DisplayImpl1")
    }
}

pub struct DisplayImplProxy<T: Display>(PhantomData<T>);
impl<T: Display> ShadowTrait for DisplayImplProxy<T> {
    type Target = T;
}
impl<T: Display> ShadowDisplay for DisplayImplProxy<T> {
    fn fmt(this: &Self::Target, f: &mut Formatter<'_>) -> Result {
        f.write_str("Display Pre ")?;
        this.fmt(f)?;
        f.write_str(" Post")?;
        Ok(())
    }
}
```

## 5. comsumer
```rust
pub struct SimpleMultipleTag;
impl ShadowTrait for SimpleMultipleTag {
    type Target = i32;
}
impl ShadowDisplayProvider for SimpleMultipleTag {
    type Impl = DisplayImplProxy<i32>;
}
impl ShadowDebugProvider for SimpleMultipleTag {
    type Impl = DebugImpl1;
}

#[test]
fn test_simple_multiple() {
    let num = 42;
    let a1 = Wrap::<SimpleMultipleTag>::wrap_ref(&num);
    assert_eq!(format!("{a1}"), "Display Pre 42 Post");
    assert_eq!(format!("{a1:?}"), "DebugImpl1");
}
```
