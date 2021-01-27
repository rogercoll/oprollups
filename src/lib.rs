pub mod batch;
pub mod transaction;
pub mod mktree;
pub mod account;
use std::collections::HashMap;


pub fn hello() -> String {
    return "Hello".to_string()
}

pub struct Accounts {
    hash_map: HashMap<u32, account::Account>,
}

impl Accounts {
    pub fn new() -> Accounts {
        Accounts {hash_map: HashMap::new()}
    }
    pub fn add(&mut self, user: account::Account) {
        self.hash_map.insert(user.addr, user);
    }
    pub fn merkle_tree(&self) -> String {
        let mut leafs:Vec<[u8;32]> = vec![[0u8;32]; self.hash_map.len()];    
        for (addr, acc) in &self.hash_map {
            leafs[acc.id as usize] = *acc.hash().as_bytes();
        }
        mktree::root2(leafs)
    }
}