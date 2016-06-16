use std::collections::HashSet;
use std::iter;
use std::hash::{Hash, SipHasher, Hasher, BuildHasher, BuildHasherDefault};

pub struct RendezvousHash<N, H> {
    store: HashSet<N>,
    hash_builder: H,
}

impl<N: Eq + Hash> RendezvousHash<N, BuildHasherDefault<SipHasher>> {
    pub fn of<I: iter::IntoIterator<Item = N>>
        (src: I)
         -> RendezvousHash<N, BuildHasherDefault<SipHasher>> {
        let store = src.into_iter().collect();
        RendezvousHash {
            store: store,
            hash_builder: BuildHasherDefault::default(),
        }
    }
}

impl<N: Eq + Hash, H: BuildHasher> RendezvousHash<N, H> {
    pub fn bucket_for<K: Hash>(&self, key: K) -> Option<&N> {
        self.store.iter().max_by_key(|node| self.to_hash(node, &key))
    }

    fn to_hash<K: Hash>(&self, node: &N, key: K) -> u64 {
        let mut h = self.hash_builder.build_hasher();
        node.hash(&mut h);
        key.hash(&mut h);
        h.finish()
    }
}


#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use super::RendezvousHash;
    #[test]
    fn distribution_should_be_roughly_equal() {
        let nbuckets = 8u8;
        let nkeys = 1024u64;

        let mut buckets = BTreeMap::new();
        let hrw = RendezvousHash::of(0..nbuckets);
        for k in 0..nkeys {
            let bucket = hrw.bucket_for(format!("{}", k)).expect("bucket");
            *buckets.entry(bucket.clone()).or_insert(0) += 1;
        }

        println!("Buckets:{:?}", buckets);

        let min = buckets.values().cloned().min().expect("min value");
        let max = buckets.values().cloned().max().expect("max value");
        let exp_min = nkeys / nbuckets as u64 / 2;
        let exp_max = nkeys * 2 / nbuckets as u64;
        println!("min:{} > {}; max:{} < {}", min, exp_min, max, exp_max);
        assert!(min > exp_min);
        assert!(max < exp_max);
    }

    #[test]
    fn should_be_deterministic() {
        let nbuckets = 8u8;
        let nkeys = 64u64;

        let hrw_a = RendezvousHash::of(0..nbuckets);
        let hrw_b = RendezvousHash::of(0..nbuckets);

        let buckets_a = (0..nkeys).map(|k| hrw_a.bucket_for(k)).collect::<Vec<_>>();
        let buckets_b = (0..nkeys).map(|k| hrw_b.bucket_for(k)).collect::<Vec<_>>();

        assert_eq!(buckets_a, buckets_b);
    }

    #[test]
    fn node_addition_should_preserve_most_keys() {
        let before = ["fred", "barney", "wilma", "betty"];
        let after = ["fred", "barney", "wilma", "betty", "bambam"];
        let nkeys = 64usize;

        let hrw_a = RendezvousHash::of(&before);
        let hrw_b = RendezvousHash::of(&after);

        let changed = (0..nkeys).filter(|k| hrw_a.bucket_for(k) != hrw_b.bucket_for(k)).count();
        println!("changed items: {:?}/{:?}", changed, nkeys);
        assert!(changed < nkeys / 4);
    }

}
