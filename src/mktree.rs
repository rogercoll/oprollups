use std::hash::Hasher;
use crypto::sha3::{Sha3, Sha3Mode};
use crypto::digest::Digest;
use merkletree::hash::{Algorithm};
use merkletree::merkle::MerkleTree;
use merkletree::store::VecStore;


pub struct ExampleAlgorithm(Sha3);

impl ExampleAlgorithm {
    pub fn new() -> ExampleAlgorithm {
        ExampleAlgorithm(Sha3::new(Sha3Mode::Sha3_256))
    }
}

impl Default for ExampleAlgorithm {
    fn default() -> ExampleAlgorithm {
        ExampleAlgorithm::new()
    }
}

impl Hasher for ExampleAlgorithm {
    #[inline]
    fn write(&mut self, msg: &[u8]) {
        self.0.input(msg)
    }

    #[inline]
    fn finish(&self) -> u64 {
        unimplemented!()
    }
}

impl Algorithm<[u8; 32]> for ExampleAlgorithm {
    #[inline]
    fn hash(&mut self) -> [u8; 32] {
        let mut h = [0u8; 32];
        self.0.result(&mut h);
        h
    }

    #[inline]
    fn reset(&mut self) {
        self.0.reset();
    }
}
use core::fmt::Write;


//SHA3-256
pub fn root(h1: &[u8; 32], h2: &[u8; 32], h3: &[u8; 32], h4: &[u8; 32]) -> String {
    let t: MerkleTree<[u8; 32], ExampleAlgorithm, VecStore<_>> = MerkleTree::try_from_iter(vec![*h1, *h2, *h3, *h4].into_iter().map(Ok)).unwrap();
    let root = &t.root();
    let mut s = String::with_capacity(2 * (t.root().len()));
    for byte in root {
        write!(s, "{:02X}", byte).expect("Could not convert to hexadecimal value");
    }
    s
}

//SHA3-256
pub fn root2(leafs: Vec<[u8;32]>) -> String {
    let t: MerkleTree<[u8; 32], ExampleAlgorithm, VecStore<_>> = MerkleTree::try_from_iter(leafs.into_iter().map(Ok)).unwrap();
    let root = &t.root();
    let mut s = String::with_capacity(2 * (t.root().len()));
    for byte in root {
        write!(s, "{:02X}", byte).expect("Could not convert to hexadecimal value");
    }
    s
}