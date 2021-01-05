//! A crate that allows for compile time reflection of tagged types.

#![no_std]
#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(generic_associated_types)]
#![warn(clippy::all)]
#![warn(missing_docs)]

pub mod drop;
pub mod fields;

pub use memoffset;
