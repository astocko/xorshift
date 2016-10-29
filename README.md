# xorshift

Rust crate implementing the high performance splitmix64, xoroshiro128+, xorshift128+, and xorshift1024* PRNGs. Derived from their respective public-domain C implementations.

See [COPYRIGHT](COPYRIGHT) for details.

# algorithms

Please see [http://xoroshiro.di.unimi.it](http://xoroshiro.di.unimi.it/) for an overview of the PRNGs and their preferred use cases. For parallel simulations, xorshift1024* is recommended, otherwise xoroshiro128+. splitmix64 is convenient for seeding PRNG states.
