//! To avoid duplicating derived implementations for the same type, derived implementations are
//! addressed by a hash of their type content. However, different derived implementations can be
//! reused based on different properties of the type. For example:
//!
//! - `Eq` does not care about surface type representations; its derived implementations can be
//!   uniquely addressed by the [`Layout`][crate::layout::Layout] of a type.
//! - `Encoding` must care about surface type representations; for example, `{ a: "" }` and
//!   `{ b: "" }` have different derived implementations. However, it does not need to distinguish
//!   between e.g. required and optional record fields.
//! - `Decoding` is like encoding, but has some differences. For one, it *does* need to distinguish
//!   between required and optional record fields.
//!
//! For these reasons the content hashing is based on a [`Strategy`] as well.

use roc_types::subs::{Subs, Variable};

#[derive(Hash)]
enum Strategy {
    Encoding,
    #[allow(unused)]
    Decoding,
}

#[derive(Hash)]
struct DeriverHashImpl<H>
where
    H: std::hash::Hash,
{
    strategy: Strategy,
    hash: H,
}

pub trait DeriverHash: std::hash::Hash {}

impl<H> DeriverHash for DeriverHashImpl<H> where H: std::hash::Hash {}

pub struct EncodingHash;
impl EncodingHash {
    pub fn from_var(subs: &Subs, var: Variable) -> impl DeriverHash + '_ {
        DeriverHashImpl {
            strategy: Strategy::Encoding,
            hash: super::encoding::FlatEncodable::from_var(subs, var),
        }
    }
}
