// Written by Alexander Stocko <as@coder.gg>
//
// To the extent possible under law, the author has dedicated all copyright
// and related and neighboring rights to this software to the public domain
// worldwide. This software is distributed without any warranty.
//
// See <LICENSE or http://creativecommons.org/publicdomain/zero/1.0/>

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
//!
//! # Examples
//! ```rust
//! extern crate time;
//! extern crate xorshift;
//!
//! use time::precise_time_ns;
//! use xorshift::{Rand, Rng, SeedableRng, SplitMix64, Xoroshiro128, Xorshift128, Xorshift1024};
//!
//! fn main() {
//!     // Use the high-resolution performance counter for seeding
//!     let now = precise_time_ns();
//!
//!     // Manually seed a Xorshift128+ PRNG
//!     let states = [now, now];
//!     let mut rng: Xorshift128 = SeedableRng::from_seed(&states[..]);
//!     println!("Xorshift128+ random u64: {}", rng.next_u64());
//!
//!     // Use a SplitMix64 PRNG to seed a Xoroshiro128+ PRNG
//!     let mut sm: SplitMix64 = SeedableRng::from_seed(now);
//!     let mut rng: Xoroshiro128 = Rand::rand(&mut sm);
//!     println!("Xoroshiro128+ random u64: {}", rng.next_u64());
//!
//!     let mut rng: Xorshift1024 = Rand::rand(&mut sm);
//!     println!("Xorshift1024* random u64: {}", rng.next_u64());
//!
//!     // Generate 20 random u32s
//!     let vals = rng.gen_iter::<u32>().take(20).collect::<Vec<u32>>();
//!     println!("Xorshift1024* random u32: {:?}", vals);
//!
//!     // Generate 50 random u64s
//!     let vals = rng.gen_iter::<u64>().take(50).collect::<Vec<u64>>();
//!     println!("Xorshift1024* random u64: {:?}", vals);
//! }
//!
//! ```
//!
//! # Parallelism
//! Applications with little parallelism, should use the Xoroshiro128+ generator.
//! For large scale parallel computations, use Xorshift1024*. Either use the
//! `thread_rng()` function to create generators with the same seed but incremented
//! jump states or explicitly use the jump function to forward generator
//! state.
//!
//! ```rust
//! extern crate xorshift;
//!
//! use std::thread;
//! use xorshift::{Rng, Xorshift1024};
//! use xorshift::thread_rng;
//!
//! fn main() {
//!     let mut threads = Vec::new();
//!
//!     for i in 0..17 {
//!         threads.push(thread::spawn(move || {
//!             let mut r: Xorshift1024 = thread_rng();
//!             println!("Thread: {}, random u64: {}", i, r.next_u64());
//!         }));
//!     }
//!
//!     for child in threads {
//!         let _ = child.join();
//!     }
//! }
//! ```
//!
//!
//! ```rust
//! extern crate time;
//! extern crate xorshift;
//!
//! use std::thread;
//! use time::precise_time_ns;
//! use xorshift::{Rand, Rng, RngJump, SeedableRng, SplitMix64, Xorshift1024};
//!
//! fn main() {
//!     // Use the high-resolution performance counter for seeding
//!     let now = precise_time_ns();
//!
//!     let mut sm: SplitMix64 = SeedableRng::from_seed(now);
//!     let rng: Xorshift1024 = Rand::rand(&mut sm);
//!
//!     let mut threads = Vec::new();
//!
//!     for i in 0..17 {
//!         threads.push(thread::spawn(move || {
//!             let mut r = rng;
//!             r.jump(i);
//!             println!("Thread: {}, random u64: {}", i, r.next_u64());
//!         }));
//!     }
//!
//!     for child in threads {
//!         let _ = child.join();
//!     }
//! }
//! ```
//!

#[macro_use]
extern crate lazy_static;

extern crate rand;

pub mod splitmix64;
pub mod xoroshiro128;
pub mod xorshift128;
pub mod xorshift1024;

pub use splitmix64::SplitMix64;
pub use xoroshiro128::Xoroshiro128;
pub use xorshift128::Xorshift128;
pub use xorshift1024::Xorshift1024;

pub use rand::{Rand, Rng, SeedableRng, StdRng};

use std::sync::atomic::{AtomicUsize, Ordering};

/// A random number generator with jumpable state.
pub trait RngJump {
    /// Forward the state of the random number generator.
    ///
    /// When using the random number generator for parallel computations,
    /// jump the state to avoid biased generation.
    fn jump(&mut self, count: usize);
}


/// Create a jumpable random number generator. Each call increments
/// the generator jump state.
pub fn thread_rng <'a, T: Rand+Rng+RngJump+SeedableRng<&'a [u64]>>() -> T {
    lazy_static! {
        static ref THREAD_RNG_STATE : Vec<u64> = {
            match StdRng::new() {
                Ok(mut r) => r.gen_iter::<u64>().take(16).collect::<Vec<u64>>(),
                Err(e) => panic!("could not initialize seeding rng: {}", e)
            }
        };
        static ref THREAD_RNG_INSTANCE : AtomicUsize = AtomicUsize::new(0);
    };

    let mut rng:T = SeedableRng::from_seed(&(*THREAD_RNG_STATE)[..]);
    rng.jump((*THREAD_RNG_INSTANCE).fetch_add(1, Ordering::SeqCst));

    rng
}


// Taken from the lib.rs in the rand crate.
#[cfg(test)]
mod test {
    pub fn iter_eq<I, J>(i: I, j: J) -> bool
        where I: IntoIterator, J: IntoIterator<Item = I::Item>, I::Item: Eq {

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
