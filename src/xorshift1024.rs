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

const STATE_SIZE: usize = 16;

#[derive(Copy, Clone)]
pub struct Xorshift1024 {
    state: [u64; 16],
    p: usize,
}

static EMPTY: Xorshift1024 = Xorshift1024 {
    state: [0; 16],
    p: 0,
};
static JUMP: [u64; 16] = [0x84242f96eca9c41d,
                          0xa3c65b8776f96855,
                          0x5b34a39f070b5837,
                          0x4489affce4f31a1e,
                          0x2ffeeb0a48316f40,
                          0xdc2d9891fe68c022,
                          0x3659132bb12fea70,
                          0xaac17d8efa43cab8,
                          0xc4cb815590989b13,
                          0x5ee975283d71c93b,
                          0x691548c86c1bd540,
                          0x7910c41d10a1e6a5,
                          0x0b5fc64563b3e2a8,
                          0x047f7684e9fc949d,
                          0xb99181f2d8f685ca,
                          0x284600e3f30e38c3];


impl Rng for Xorshift1024 {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let s0 = self.state[self.p];
        self.p = (self.p + 1) & 15;
        let mut s1 = self.state[self.p];

        s1 ^= s1 << 31;
        self.state[self.p] = s1 ^ s0 ^ (s1 >> 11) ^ (s0 >> 30);

        (w(self.state[self.p]) * w(1181783497276652981_u64)).0
    }
}

impl<'a> SeedableRng<&'a [u64]> for Xorshift1024 {
    fn reseed(&mut self, seed: &'a [u64]) {
        if seed.len() < 16 {
            panic!("Xorshift1024 seed needs at least 16 u64s for seeding.");
        }

        for (index, element) in seed.iter().enumerate() {
            self.state[index] = *element;
        }
    }

    fn from_seed(seed: &'a [u64]) -> Xorshift1024 {
        let mut rng = EMPTY;
        rng.reseed(seed);
        rng
    }
}

impl Rand for Xorshift1024 {
    fn rand<R: Rng>(other: &mut R) -> Xorshift1024 {
        let mut key: [u64; STATE_SIZE] = [0; STATE_SIZE];
        for word in key.iter_mut() {
            *word = other.gen();
        }
        SeedableRng::from_seed(&key[..])
    }
}

impl RngJump for Xorshift1024 {
    fn jump(&mut self, count: usize) {
        let mut s = self.state;
        let p = self.p;

        for _ in 0..count {
            let mut t: [u64; 16] = [0; 16];
            for i in 0..JUMP.len() {
                for b in 0..64 {
                    if (JUMP[i] & 1 << b) != 0 {
                        for j in 0..16 {
                            t[j] ^= s[(j + p) & 15];
                        }
                    }
                    self.next_u64();
                }
            }

            for j in 0..16 {
                s[(j + p) & 15] = t[j];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::{Rng, SeedableRng};
    use super::Xorshift1024;
    #[test]
    fn test() {
        // Calculated from reference implementation
        // https://github.com/astocko/xorshift-cpp
        let seed: u64 = 1477777179826044140;
        let t_vals: Vec<u64> = vec![14360464905097655832,
                                    10515520027797512354,
                                    12277485841648819968,
                                    5975068082386226908,
                                    14360464905097655832,
                                    10515520027797512354,
                                    12277485841648819968,
                                    5975068082386226908,
                                    14360464905097655832,
                                    10515520027797512354,
                                    12277485841648819968,
                                    5975068082386226908,
                                    14360464905097655832,
                                    10515520027797512354,
                                    12277485841648819968,
                                    5975068082386226908,
                                    16155457212423715006,
                                    16973689320641693688,
                                    11981506001797128964,
                                    13241400995114197981,
                                    2158488016667357978,
                                    3377935610872016481,
                                    12277485841648819968,
                                    5975068082386226908,
                                    16155457212423715006,
                                    16973689320641693688,
                                    11981506001797128964,
                                    13241400995114197981,
                                    2158488016667357978,
                                    3377935610872016481,
                                    12277485841648819968,
                                    5975068082386226908,
                                    3862476215600981850,
                                    666405138486472370,
                                    2467704680056122713,
                                    18070567468833369740,
                                    14135306694933672725,
                                    3377935610872016481,
                                    12277485841648819968,
                                    5975068082386226908,
                                    3862476215600981850,
                                    666405138486472370,
                                    2467704680056122713,
                                    18070567468833369740,
                                    14135306694933672725,
                                    3377935610872016481,
                                    12277485841648819968,
                                    5975068082386226908,
                                    812945179660782235,
                                    14943324017293890156];

        let states = [seed; 16];
        let mut rng: Xorshift1024 = SeedableRng::from_seed(&states[..]);
        let vals = rng.gen_iter::<u64>().take(t_vals.len()).collect::<Vec<u64>>();
        assert!(::test::iter_eq(t_vals, vals));
    }
}
