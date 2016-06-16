#![feature(test)]
extern crate test;
extern crate rendezvous_hash;
extern crate twox_hash;

use std::hash::{Hash,Hasher,SipHasher, BuildHasherDefault};
use rendezvous_hash::RendezvousHash;
use twox_hash::XxHash;

#[bench] fn of_siphash_usize_2pow01(b: &mut test::Bencher)  { bench_siphash(0..(1<<1), 42, b); }
#[bench] fn of_siphash_usize_2pow06(b: &mut test::Bencher)  { bench_siphash(0..(1<<6), 42, b); }
#[bench] fn of_siphash_usize_2pow07(b: &mut test::Bencher)  { bench_siphash(0..(1<<7), 42, b); }
#[bench] fn of_siphash_usize_2pow08(b: &mut test::Bencher)  { bench_siphash(0..(1<<8), 42, b); }
#[bench] fn of_siphash_usize_2pow09(b: &mut test::Bencher)  { bench_siphash(0..(1<<9), 42, b); }
#[bench] fn of_siphash_usize_2pow10(b: &mut test::Bencher)  { bench_siphash(0..(1<<10), 42, b); }
#[bench] fn of_siphash_usize_2pow16(b: &mut test::Bencher)  { bench_siphash(0..(1<<16), 42, b); }

fn bench_siphash<I: Iterator<Item=N>, N: Eq + Hash, K: Hash>(nodes: I, key: K, b: &mut test::Bencher)  {
    let rh = RendezvousHash::of(nodes);
    b.iter(|| {
        rh.bucket_for(&key)
    })
}

#[bench] fn of_xxhash_usize_2pow01(b: &mut test::Bencher)  { bench_xxhash(0..(1<<1), 42, b); }
#[bench] fn of_xxhash_usize_2pow06(b: &mut test::Bencher)  { bench_xxhash(0..(1<<6), 42, b); }
#[bench] fn of_xxhash_usize_2pow07(b: &mut test::Bencher)  { bench_xxhash(0..(1<<7), 42, b); }
#[bench] fn of_xxhash_usize_2pow08(b: &mut test::Bencher)  { bench_xxhash(0..(1<<8), 42, b); }
#[bench] fn of_xxhash_usize_2pow09(b: &mut test::Bencher)  { bench_xxhash(0..(1<<9), 42, b); }
#[bench] fn of_xxhash_usize_2pow10(b: &mut test::Bencher)  { bench_xxhash(0..(1<<10), 42, b); }
#[bench] fn of_xxhash_usize_2pow16(b: &mut test::Bencher)  { bench_xxhash(0..(1<<16), 42, b); }

fn bench_xxhash<I: Iterator<Item=N>, N: Eq + Hash, K: Hash>(nodes: I, key: K, b: &mut test::Bencher)  {
    let hash = BuildHasherDefault::<XxHash>::default();
    let rh = RendezvousHash::of_hasher(hash, nodes);
    b.iter(|| {
        rh.bucket_for(&key)
    })
}
