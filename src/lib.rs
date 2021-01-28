pub mod batch;
pub mod transaction;
pub mod mktree;
pub mod account;
use std::collections::HashMap;


pub fn hello() -> String {
    return "Hello".to_string()
}

pub struct Accounts <'a> {
    hash_map: HashMap<& 'a u32, & 'a account::Account>,
}

pub struct Transactions <'a> {
    b_transactions: Vec<& 'a transaction::Transaction>,
}

impl <'a>Accounts<'a> {
    pub fn new() -> Accounts<'a> {
        Accounts {hash_map: HashMap::new()}
    }
    pub fn add(&mut self, user: &'a account::Account) {
        self.hash_map.insert(&user.addr, user);
    }
    pub fn merkle_tree(&self) -> String {
        let mut leafs:Vec<[u8;32]> = vec![[0u8;32]; self.hash_map.len()];    
        for (_, acc) in &self.hash_map {
            leafs[acc.id as usize] = *acc.hash().as_bytes();
        }
        mktree::root2(leafs)
    }
}

impl <'a>Transactions<'a> {
    pub fn new() -> Transactions<'a> {
        Transactions {b_transactions: Vec::new()}
    }
    pub fn add(&mut self, t: &'a transaction::Transaction) {
        self.b_transactions.push(&t);
    }
}