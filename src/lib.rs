// Copyright 2016 Alexander Stocko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implementation of the high performance xoroshiro128+, xorshift128+, xorshift1024*, and splitmix64 pseudo random number generators.
//!
//! Implements the `Rand`, `Rng`, and `SeedableRng` traits from the [rand crate](https://crates.io/crates/rand).
//!
//! # Usage
//! ```toml
//! [dependencies]
//! xorshift = "0.1"
//! ```
//! ```rust
//! extern crate xorshift;
//! ```
//! # Examples
//! 

extern crate rand;

pub mod splitmix64;
pub mod xoroshiro128;
pub mod xorshift128;
pub mod xorshift1024;

pub use splitmix64::SplitMix64;
pub use xoroshiro128::Xoroshiro128;
pub use xorshift128::Xorshift128;
pub use xorshift1024::Xorshift1024;

pub use rand::{Rand, Rng, SeedableRng};

/// A random number generator with jumpable state.
pub trait RngJump {
    /// Forward the state of the random number generator.
    ///
    /// When using the random number generator for parallel computations,
    /// jump the state to avoid biased generation.
    fn jump(&mut self, count: usize);
}


// Taken from the lib.rs in the rand crate.
#[cfg(test)]
mod test {
    pub fn iter_eq<I, J>(i: I, j: J) -> bool
        where I: IntoIterator,
              J: IntoIterator<Item = I::Item>,
              I::Item: Eq
    {
        // make sure the iterators have equal length
        let mut i = i.into_iter();
        let mut j = j.into_iter();
        loop {
            match (i.next(), j.next()) {
                (Some(ref ei), Some(ref ej)) if ei == ej => {}
                (None, None) => return true,
                _ => return false,
            }
        }
    }
}
