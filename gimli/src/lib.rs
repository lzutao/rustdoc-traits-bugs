#![feature(staged_api)]
#![unstable(feature = "rustc_private", issue = "none")]

#[unstable(feature = "rustc_private", issue = "none")]
pub struct EndianSlice;

#[unstable(feature = "rustc_private", issue = "none")]
impl realcore::Deref for EndianSlice {}
