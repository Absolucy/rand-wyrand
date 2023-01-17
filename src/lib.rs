// SPDX-License-Identifier: Apache-2.0 OR MIT
//! This is a library that implements the [WyRand](rand_wyrand::WyRand) PRNG for
//! the [RngCore](rand_core::RngCore) trait from `rand`. It is completely
//! `#![no_std]`, and does not require the Rust standard library.
//!
//! WyRand is an extremely fast, stable, and solid PRNG. While not
//! cryptographically secure, it still passes the BigCrush and practrand tests,
//! making it a solid choice for non-secure applications. (Basically, just don't
//! use it to like, generate cryptographic keys or passwords or whatever)
//!
//! ## Examples
//!
//! ### Generate random number from 1 to 100
//!
//! ```rust
//! use rand::{Rng, SeedableRng};
//! use rand_wyrand::WyRand;
//!
//! let mut wyrand = WyRand::from_entropy();
//! let mut bytes = [0_u8; 64];
//! wyrand.fill(&mut bytes);
//! println!("Random bytes: {bytes:?}");
//! ```
//!
//! ### Generate random bytes
//!
//! ```rust
//! use rand::{Rng, SeedableRng};
//! use rand_wyrand::WyRand;
//!
//! let mut wyrand = WyRand::from_entropy();
//! println!("Random number from 1 to 100: {}", wyrand.gen_range(1..=100));
//! ```
//!
//! ### Generate random string
//!
//! ```rust
//! use rand::{distributions::Alphanumeric, Rng, SeedableRng};
//! use rand_wyrand::WyRand;
//!
//! let mut wyrand = WyRand::from_entropy();
//! let rand_string: String = wyrand
//! 	.sample_iter(&Alphanumeric)
//! 	.take(16)
//! 	.map(char::from)
//! 	.collect();
//! println!("Random string: {rand_string}")
//! ```
//!
//! ## License
//!
//! `rand-wyrand` is licensed under either the [Apache
//! License](LICENSE-APACHE.md) or the [MIT License](LICENSE-MIT.md), at your
//! choice.
#![no_std]
#![forbid(unsafe_code)]
#![deny(clippy::perf, clippy::style, clippy::correctness, clippy::complexity)]
#![allow(clippy::tabs_in_doc_comments)]
use core::fmt::Debug;

use rand_core::{impls::fill_bytes_via_next, Error, RngCore, SeedableRng};

#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

/// An instance of the [WyRand](https://github.com/wangyi-fudan/wyhash) random number generator.
///
/// While not cryptographically secure, WyRand is solid enough to pass
/// the [BigCrush](https://en.wikipedia.org/wiki/TestU01) and
/// [practrand](http://pracrand.sourceforge.net/) tests, while being extremely fast, making it ideal
/// for non-secure uses.
///
/// ## Examples
///
/// ### Generate random number from 1 to 100
///
/// ```rust
/// use rand::{Rng, SeedableRng};
/// use rand_wyrand::WyRand;
///
/// let mut wyrand = WyRand::from_entropy();
/// let mut bytes = [0_u8; 64];
/// wyrand.fill(&mut bytes);
/// println!("Random bytes: {bytes:?}");
/// ```
///
/// ### Generate random bytes
///
/// ```rust
/// use rand::{Rng, SeedableRng};
/// use rand_wyrand::WyRand;
///
/// let mut wyrand = WyRand::from_entropy();
/// println!("Random number from 1 to 100: {}", wyrand.gen_range(1..=100));
/// ```
///
/// ### Generate random string
///
/// ```rust
/// use rand::{distributions::Alphanumeric, Rng, SeedableRng};
/// use rand_wyrand::WyRand;
///
/// let mut wyrand = WyRand::from_entropy();
/// let rand_string: String = wyrand
/// 	.sample_iter(&Alphanumeric)
/// 	.take(16)
/// 	.map(char::from)
/// 	.collect();
/// println!("Random string: {rand_string}")
/// ```
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct WyRand {
	seed: u64,
}

impl RngCore for WyRand {
	#[inline]
	fn next_u32(&mut self) -> u32 {
		self.next_u64() as u32
	}

	#[inline]
	fn next_u64(&mut self) -> u64 {
		self.seed = self.seed.wrapping_add(0xa0761d6478bd642f);
		let t: u128 = (self.seed as u128).wrapping_mul((self.seed ^ 0xe7037ed1a0b428db) as u128);
		(t.wrapping_shr(64) ^ t) as u64
	}

	#[inline]
	fn fill_bytes(&mut self, dest: &mut [u8]) {
		fill_bytes_via_next(self, dest)
	}

	#[inline]
	fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
		self.fill_bytes(dest);
		Ok(())
	}
}

impl SeedableRng for WyRand {
	type Seed = [u8; core::mem::size_of::<u64>()];

	#[inline]
	fn from_seed(seed: Self::Seed) -> Self {
		Self::seed_from_u64(u64::from_le_bytes(seed))
	}

	#[inline]
	fn seed_from_u64(state: u64) -> Self {
		Self { seed: state }
	}

	#[inline]
	fn from_rng<R: RngCore>(mut rng: R) -> Result<Self, Error> {
		Ok(Self::seed_from_u64(rng.next_u64()))
	}
}

// Custom Debug implementation that does not expose the internal state
impl Debug for WyRand {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_tuple("WyRand").finish()
	}
}

#[cfg(test)]
mod tests {
	extern crate alloc;

	use alloc::format;

	use super::*;

	#[test]
	fn no_leaking_debug() {
		let rng = WyRand::from_seed(Default::default());

		assert_eq!(format!("{:?}", rng), "WyRand");
	}

	#[cfg(feature = "serde1")]
	#[test]
	fn serde_tokens() {
		use serde_test::{assert_tokens, Token};

		let rng = WyRand::seed_from_u64(12345);

		assert_tokens(&rng, &[
			Token::Struct {
				name: "WyRand",
				len: 1,
			},
			Token::BorrowedStr("seed"),
			Token::U64(12345),
			Token::StructEnd,
		]);
	}
}
