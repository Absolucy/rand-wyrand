// SPDX-License-Identifier: Apache-2.0 OR MIT
#![no_std]
use rand_core::{impls::fill_bytes_via_next, Error, RngCore, SeedableRng};

/// An instance of the [WyRand](https://github.com/wangyi-fudan/wyhash) random number generator.
///
/// While not cryptographically secure, WyRand is solid enough to pass
/// the [BigCrush](https://en.wikipedia.org/wiki/TestU01) and
/// [practrand](http://pracrand.sourceforge.net/) tests, while being extremely fast, making it ideal
/// for non-secure uses.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
