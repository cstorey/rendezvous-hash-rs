#![feature(test)]
extern crate test;
extern crate rendezvous_hash;

use std::io::Cursor;
use std::default::Default;
use std::hash::Hash;
use rendezvous_hash::RendezvousHash;

#[bench] fn of_usize_2pow01(b: &mut test::Bencher)  { of_size(0..(1<<1), 42, b); }
#[bench] fn of_usize_2pow06(b: &mut test::Bencher)  { of_size(0..(1<<6), 42, b); }
#[bench] fn of_usize_2pow07(b: &mut test::Bencher)  { of_size(0..(1<<7), 42, b); }
#[bench] fn of_usize_2pow08(b: &mut test::Bencher)  { of_size(0..(1<<8), 42, b); }
#[bench] fn of_usize_2pow09(b: &mut test::Bencher)  { of_size(0..(1<<9), 42, b); }
#[bench] fn of_usize_2pow10(b: &mut test::Bencher)  { of_size(0..(1<<10), 42, b); }
#[bench] fn of_usize_2pow16(b: &mut test::Bencher)  { of_size(0..(1<<16), 42, b); }

fn of_size<I: Iterator<Item=N>, N: Eq + Hash, K: Hash>(nodes: I, key: K, b: &mut test::Bencher)  { 
    let mut rh = RendezvousHash::of(nodes);
    b.iter(|| {
        rh.bucket_for(&key)
    })
}
