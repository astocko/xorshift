// Copyright 2016 Alexander Stocko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::num::Wrapping as w;
use rand::{Rand, Rng, SeedableRng};
use RngJump;

const STATE_SIZE: usize = 2;

#[derive(Copy, Clone)]
pub struct Xorshift128([u64; 2]);

static EMPTY: Xorshift128 = Xorshift128([0, 0]);
static JUMP: [u64; 2] = [0x8a5cd789635d2dff, 0x121fd2155c472f96];

impl Rng for Xorshift128 {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let mut s1 = w(self.0[0]);
        let s0 = w(self.0[1]);
        let result = s0 + s1;
        self.0[0] = s0.0;
        s1 ^= s1 << 23;
        self.0[1] = (s1 ^ s0 ^ (s1 >> 18) ^ (s0 >> 5)).0;
        result.0
    }
}

impl<'a> SeedableRng<&'a [u64]> for Xorshift128 {
    fn reseed(&mut self, seed: &'a [u64]) {
        if seed.len() < 2 {
            panic!("Xorshift128 seed needs at least two u64s for seeding.");
        }
        self.0[0] = seed[0];
        self.0[1] = seed[1];
    }

    fn from_seed(seed: &'a [u64]) -> Xorshift128 {
        let mut rng = EMPTY;
        rng.reseed(seed);
        rng
    }
}

impl Rand for Xorshift128 {
    fn rand<R: Rng>(other: &mut R) -> Xorshift128 {
        let mut key: [u64; STATE_SIZE] = [0; STATE_SIZE];
        for word in key.iter_mut() {
            *word = other.gen();
        }
        SeedableRng::from_seed(&key[..])
    }
}

impl RngJump for Xorshift128 {
    fn jump(&mut self, count: usize) {
        for _ in 0..count {
            let mut s0: u64 = 0;
            let mut s1: u64 = 0;

            for i in 0..JUMP.len() {
                for b in 0..64 {
                    if (JUMP[i] & 1 << b) != 0 {
                        s0 ^= self.0[0];
                        s1 ^= self.0[1];
                    }
                    self.next_u64();
                }
            }
            self.0[0] = s0;
            self.0[1] = s1;
        }
    }
}


#[cfg(test)]
mod tests {
    use rand::{Rng, SeedableRng};
    use super::Xorshift128;
    #[test]
    fn test() {
        // Calculated from reference implementation
        // https://github.com/astocko/xorshift-cpp
        let seed: u64 = 1477776990746309507;
        let t_vals: Vec<u64> = vec![2955553981492619014,
                                    4599697141668829146,
                                    4670196263639928724,
                                    16937752213077027105,
                                    9891185907692497053,
                                    15201764008617186196,
                                    15346536721100407138,
                                    1632066265273679815,
                                    3374113843714423023,
                                    16609654801952101372,
                                    10179374700856453570,
                                    13415587091341440,
                                    13713531109933318529,
                                    9635993043755786933,
                                    13325653044572447755,
                                    15693762250133287478,
                                    12271064446641005509,
                                    2709845634781129372,
                                    12573435284135488980,
                                    12323032091170684494,
                                    16353258292748552418,
                                    10233934017009620357,
                                    14350043634790606694,
                                    15857154722661923587,
                                    9599170926588813820,
                                    9313747211033478552,
                                    7650530421537508985,
                                    633304507529020339,
                                    1432383473114491350,
                                    11195010954091482555,
                                    2197040232331856193,
                                    17592989984699807827,
                                    12638411464544161602,
                                    4396142987860263564,
                                    16456439119028401503,
                                    1345258822949363305,
                                    3509353510520372253,
                                    18232499665317415612,
                                    10679351282305608316,
                                    9368589195308537593,
                                    4894090245044721815,
                                    1948558019100264117,
                                    18309589142408570815,
                                    4816161030343661271,
                                    11210608633196506254,
                                    12612051789490434918,
                                    11585670264215608103,
                                    946134795473836869,
                                    9936715390587068425,
                                    4972002347465534564];

        let states = [seed, seed];
        let mut rng: Xorshift128 = SeedableRng::from_seed(&states[..]);
        let vals = rng.gen_iter::<u64>().take(t_vals.len()).collect::<Vec<u64>>();
        assert!(::test::iter_eq(t_vals, vals));
    }
}
