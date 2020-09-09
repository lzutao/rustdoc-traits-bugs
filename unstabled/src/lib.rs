#![no_std]
#![feature(staged_api)]
#![unstable(feature = "thisisnotreal", issue = "27747")]

#[unstable(feature = "asdfasdfasdfa", issue = "27747")]
pub struct Foo<T: Sized + Clone> {
    #[unstable(feature = "asdfasdfasdfa", issue = "27747")]
    bytes: [T],
}

#[unstable(feature = "asdfasdfasdfa", issue = "27747")]
pub trait Search {}

/// Helper trait for [`[T]::join`](../../std/primitive.slice.html#method.join)
#[unstable(feature = "asdfasdfasdfa", issue = "27747")]
pub trait Join<Separator> {
    #[unstable(feature = "asdfasdfasdfa", issue = "27747")]
    /// The resulting type after concatenation
    type Output: Search;

    /// Implementation of [`[T]::join`](../../std/primitive.slice.html#method.join)
    #[unstable(feature = "asdfasdfasdfa", issue = "27747")]
    fn join(slice: &Self, sep: Separator) -> Self::Output;
}

#[unstable(feature = "asdfasdfasdfa", issue = "27747")]
pub struct Bar<T> {
    bar: T,
}

impl<T> Search for Bar<T> {}

#[unstable(feature = "asdfasdfasdfa", issue = "27747")]
impl<T: Sized + Clone> Join<&T> for Foo<T> {
    type Output = Bar<T>;

    fn join(slice: &Self, sep: &T) -> Bar<T> {
        unimplemented!()
    }
}
