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
//! use typechain::{chainlink, chain};
//! 
//! chainlink!(Foo => {
//!     const foo: u32;
//! });
//! 
//! chain!(Bar => {
//!     @Foo
//!     const foo: u32;
//! });
//! 
//! chain!(Baz => {
//!     @Foo
//!     const foo: u32;
//! });
//! 
//! let bar = Bar { foo: 42 };
//! let baz = Baz { foo: 97 };
//! 
//! let foos: Vec<&Foo> = vec![&bar, &baz];
//! ```

pub use typechain_macros::*;


chainlink!(HasTags => {
    static tags: Vec<&'static str>;
});