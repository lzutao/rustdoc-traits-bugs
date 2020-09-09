#![crate_name = "unstabled"]
#![feature(staged_api)]
#![unstable(feature = "extremely_unstable", issue = "none")]

#[unstable(feature = "extremely_unstable_foo", issue = "none")]
pub struct Foo<T: Sized + Clone> {
    #[unstable(feature = "extremely_unstable_foo", issue = "none")]
    bytes: [T],
}

#[unstable(feature = "extremely_unstable_foo", issue = "none")]
pub trait Join {
    #[unstable(feature = "extremely_unstable_foo", issue = "none")]
    type Output;

    #[unstable(feature = "extremely_unstable_foo", issue = "none")]
    fn join(slice: &Self) -> Self::Output;
}

#[unstable(feature = "extremely_unstable_foo", issue = "none")]
impl<T: Sized + Clone> Join for Foo<T> {
    type Output = Vec<T>;

    fn join(slice: &Self) -> Vec<T> {
        unimplemented!()
    }
}
