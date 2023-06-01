#![deny(missing_docs)]

//! # `typechain`
//! 
//! This crate re-exports the `typechain` macros. In the
//! future, it may also contain other utilities for
//! working with `typechain`-generated code.
//! 
//! ## Usage
//! 
//! ```
//! use typechain::{chainlink, Chain};
//! 
//! #[chainlink]
//! pub trait Foo {
//!    fn foo(&self) -> u32;
//! }
//! 
//! #[derive(Chain)]
//! pub struct Bar {
//!     #[chain(Foo)]
//!     foo: u32
//! }
//! 
//! #[derive(Chain)]
//! pub struct Baz {
//!    #[chain(Foo)]
//!    foo: u32
//! }
//! 
//! let bar = Bar { foo: 42 };
//! let baz = Baz { foo: 97 };
//! 
//! let foos: Vec<&Foo> = vec![&bar, &baz];
//! ```

pub use typechain_macros::*;