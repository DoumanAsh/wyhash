//! Rust implementation of [wyhash](https://github.com/wangyi-fudan/wyhash) algorithm

#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]
#![no_std]

mod utils;

use utils::*;

use core::sync::atomic::{AtomicU64, Ordering};

const DEFAULT_SECRET: [u64; 5] = [0xa0761d6478bd642f, 0xe7037ed1a0b428db, 0x8ebc6af09c88c6e3, 0x589965cc75374cc3, 0x1d8e4e27c47d124f];
const SEED_MOD: u64 = DEFAULT_SECRET[0];

///Generates random number with specified seed.
///
///Do note that the same number is generated for each seed value.
///
///User must modify `seed` value to generate new unique value.
///
///Not intended to be used normally, instead you should use [Random](struct.Random.html) or [AtomicRandom](struct.AtomicRandom.html)
pub fn random(seed: u64) -> u64 {
    const SEED_EXTRA: u64 = DEFAULT_SECRET[1];
    let mut seed_extra = seed ^ SEED_EXTRA;

    #[cfg(target_pointer_width = "32")]
    {
        seed_extra *= (seed_extra >> 32) | (seed_extra << 32);

        (seed * ((seed >> 32) | (seed << 32 ))) ^ ((seed_extra >> 32) | (seed_extra << 32))
    }

    #[cfg(target_pointer_width = "64")]
    {
        let mut seed = seed;

        let seeds = u128::from(seed) * u128::from(seed_extra);
        seed = seeds as u64;
        seed_extra = (seeds >> 64) as u64;
        seed ^ seed_extra
    }
}

///Wyhash based PRNG.
pub struct Random {
    seed: u64,
}

impl Random {
    #[inline(always)]
    ///Creates new instance with supplied seed.
    pub const fn new(seed: u64) -> Self {
        Self {
            seed
        }
    }

    #[inline(always)]
    ///Consumes self returning current seed.
    pub const fn into_seed(self) -> u64 {
        self.seed
    }

    #[inline(always)]
    ///Generates new number
    pub fn gen(&mut self) -> u64 {
        self.seed = self.seed.wrapping_add(SEED_MOD);
        random(self.seed)
    }
}

///Atomic Wyhash based PRNG.
///
///Comparing to plain [Random](struct.Random.html) it stores seed in atomic
///allowing it to be used concurrently.
pub struct AtomicRandom {
    seed: AtomicU64,
}

impl AtomicRandom {
    #[inline(always)]
    ///Creates new instance with supplied seed.
    pub const fn new(seed: u64) -> Self {
        Self {
            seed: AtomicU64::new(seed.wrapping_add(SEED_MOD))
        }
    }

    #[inline(always)]
    ///Consumes self returning current seed.
    pub fn into_seed(self) -> u64 {
        self.seed.into_inner()
    }

    #[inline(always)]
    ///Generates new number
    pub fn gen(&self) -> u64 {
        //We increment initially on creation.
        //This way, previous value is always modified seed.
        //And `add` stores next seed value to use.
        let seed = self.seed.fetch_add(SEED_MOD, Ordering::SeqCst);
        random(seed)
    }
}

///Hashing algorithm optimized for 32 bit
pub fn hash32(mut data: &[u8], mut seed: u32) -> u32 {
    const HASH_MOD: u64 = 0x53c5ca59;
    const HASH_MOD2: u64 = 0x74743c1b;

    let mut seed_extra = data.len() as u32;
    seed ^= (data.len() >> 32) as u32;

    let mut mix = (seed as u64 ^ HASH_MOD) * (seed_extra as u64 ^ HASH_MOD2);
    seed = mix as u32;
    seed_extra = (mix >> 32) as u32;

    while data.len() > 8 {
        seed ^= read_u32(data);
        data = &data[4..];

        seed_extra ^= read_u32(data);
        data = &data[4..];

        mix = (seed as u64 ^ HASH_MOD) * (seed_extra as u64 ^ HASH_MOD2);
        seed = mix as u32;
        seed_extra = (mix >> 32) as u32;
    }

    if data.len() >= 4 {
        seed ^= read_u32(data);
        data = &data[data.len() - 4..];

        seed_extra ^= read_u32(data);
    } else if data.len() > 0 {
        seed ^= read_part_u32(data);
    }

    mix = (seed as u64 ^ HASH_MOD) * (seed_extra as u64 ^ HASH_MOD2);
    seed = mix as u32;
    seed_extra = (mix >> 32) as u32;

    mix = (seed as u64 ^ HASH_MOD) * (seed_extra as u64 ^ HASH_MOD2);
    seed = mix as u32;
    seed_extra = (mix >> 32) as u32;

    seed ^ seed_extra
}

#[inline(always)]
///Alias to `hash` with `DEFAULT_SECRET`
pub fn def_hash(data: &[u8], seed: u64) -> u64 {
    hash(data, seed, &DEFAULT_SECRET)
}

///Hashing algorithm optimized for small input (<= 64).
pub fn hash(mut data: &[u8], mut seed: u64, secret: &[u64; 5]) -> u64 {
    let len = data.len() as u64;
    #[cold]
    fn unlikely_branch<'a>(mut data: &'a [u8], seed: &mut u64, secret: &[u64; 5]) -> &'a [u8] {
        let mut seed_extra = *seed;

        loop {
            *seed = mix(read_u64(data, 0) ^ secret[1], read_u64(data, 8) ^ *seed) ^ mix(read_u64(data, 16) ^ secret[2], read_u64(data, 24) ^ *seed);
            seed_extra = mix(read_u64(data, 32) ^ secret[3], read_u64(data, 40) ^ seed_extra) ^ mix(read_u64(data, 48) ^ secret[4], read_u64(data, 56) ^ seed_extra);

            data = &data[64..];
            if data.len() <= 64 {
                break;
            }
        }

        data
    }

    if data.len() > 64 {
        data = unlikely_branch(data, &mut seed, secret);
    }

    while data.len() > 16 {
        seed = mix(read_u64(data, 0) ^ secret[1], read_u64(data, 8) ^ seed);

        data = &data[16..]
    }

    if data.len() > 8 {
        mix(secret[1] ^ len, mix(read_u64(data, 0) ^ secret[0], read_u64(data, data.len() as isize - 8) ^ seed))
    } else if data.len() >= 4 {
        mix(secret[1] ^ len, mix(read_u32(data) as u64 ^ secret[0], read_u32(&data[data.len() - 4..]) as u64 ^ seed))
    } else if data.len() > 0 {
        mix(secret[1] ^ len, mix(read_part_u32(data) as u64 ^ secret[0], seed))
    } else {
        mix(secret[1] ^ len, mix(secret[0], seed))
    }
}
