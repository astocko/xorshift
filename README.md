# xorshift

Rust crate implementing the high performance splitmix64, xoroshiro128+, xorshift128+, and xorshift1024* PRNGs. Derived from their respective public-domain C implementations. See [COPYRIGHT](COPYRIGHT) for details.

[![Build Status](https://travis-ci.org/astocko/xorshift.svg?branch=master)](https://travis-ci.org/astocko/xorshift)
[![crates.io page](https://img.shields.io/crates/v/xorshift.svg)](https://crates.io/crates/xorshift)

[Documentation](https://docs.coder.gg/xorshift/xorshift)


# Algorithms

Please see [http://xoroshiro.di.unimi.it](http://xoroshiro.di.unimi.it/) for an overview of the PRNGs and their preferred use cases. For parallel simulations, xorshift1024* is recommended, otherwise xoroshiro128+. splitmix64 is convenient for seeding PRNG states.

# Usage
```toml
[dependencies]
xorshift = "0.1"
```
```rust
extern crate xorshift;
```

# Examples
```rust
extern crate time;
extern crate xorshift;

use time::precise_time_ns;
use xorshift::{Rand, Rng, SeedableRng, SplitMix64, Xoroshiro128, Xorshift128, Xorshift1024};

fn main() {
    // Use the high-resolution performance counter for seeding
    let now = precise_time_ns();

    // Manually seed a Xorshift128+ PRNG
    let states = [now, now];
    let mut rng: Xorshift128 = SeedableRng::from_seed(&states[..]);
    println!("Xorshift128+ random u64: {}", rng.next_u64());

    // Use a SplitMix64 PRNG to seed a Xoroshiro128+ PRNG
    let mut sm: SplitMix64 = SeedableRng::from_seed(now);
    let mut rng: Xoroshiro128 = Rand::rand(&mut sm);
    println!("Xoroshiro128+ random u64: {}", rng.next_u64());

    let mut rng: Xorshift1024 = Rand::rand(&mut sm);
    println!("Xorshift1024* random u64: {}", rng.next_u64());

    // Generate 20 random u32s
    let vals = rng.gen_iter::<u32>().take(20).collect::<Vec<u32>>();
    println!("Xorshift1024* random u32: {:?}", vals);
}

```

# Parallelism
Applications with little parallelism, should use the Xoroshiro128+ generator.
For large scale parallel computations, use Xorshift1024*. Either use the
`thread_rng()` function to create generators with the same seed but incremented
jump states or explicitly use the jump function to forward generator
state.

```rust
extern crate xorshift;

use std::thread;
use xorshift::{Rng, Xorshift1024};
use xorshift::thread_rng;

fn main() {
    let mut threads = Vec::new();

    for i in 0..17 {
        threads.push(thread::spawn(move || {
            let mut r: Xorshift1024 = thread_rng();
            println!("Thread: {}, random u64: {}", i, r.next_u64());
        }));
    }

    for child in threads {
        let _ = child.join();
    }
}
```


```rust
extern crate time;
extern crate xorshift;

use std::thread;
use time::precise_time_ns;
use xorshift::{Rand, Rng, RngJump, SeedableRng, SplitMix64, Xorshift1024};

fn main() {
    // Use the high-resolution performance counter for seeding
    let now = precise_time_ns();

    let mut sm: SplitMix64 = SeedableRng::from_seed(now);
    let rng: Xorshift1024 = Rand::rand(&mut sm);

    let mut threads = Vec::new();

    for i in 0..17 {
        threads.push(thread::spawn(move || {
            let mut r = rng;
            r.jump(i);
            println!("Thread: {}, random u64: {}", i, r.next_u64());
        }));
    }

    for child in threads {
        let _ = child.join();
    }
}
```

