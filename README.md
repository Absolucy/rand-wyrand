# rand-wyrand

This is a library that implements the [WyRand](rand_wyrand::WyRand) PRNG for
the [RngCore](rand_core::RngCore) trait from `rand`. It is completely
`#![no_std]`, and does not require the Rust standard library.

WyRand is an extremely fast, stable, and solid PRNG. While not
cryptographically secure, it still passes the BigCrush and practrand tests,
making it a solid choice for non-secure applications. (Basically, just don't
use it to like, generate cryptographic keys or passwords or whatever)

### Examples

#### Generate random number from 1 to 100

```rust
use rand::{Rng, SeedableRng};
use rand_wyrand::WyRand;

let mut wyrand = WyRand::from_entropy();
let mut bytes = [0_u8; 64];
wyrand.fill(&mut bytes);
println!("Random bytes: {bytes:?}");
```

#### Generate random bytes

```rust
use rand::{Rng, SeedableRng};
use rand_wyrand::WyRand;

let mut wyrand = WyRand::from_entropy();
println!("Random number from 1 to 100: {}", wyrand.gen_range(1..=100));
```

#### Generate random string

```rust
use rand::{distributions::Alphanumeric, Rng, SeedableRng};
use rand_wyrand::WyRand;

let mut wyrand = WyRand::from_entropy();
let rand_string: String = wyrand
	.sample_iter(&Alphanumeric)
	.take(16)
	.map(char::from)
	.collect();
println!("Random string: {rand_string}")
```

### License

`rand-wyrand` is licensed under either the [Apache
License](LICENSE-APACHE.md) or the [MIT License](LICENSE-MIT.md), at your
choice.

License: Apache-2.0 OR MIT
