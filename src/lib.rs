pub mod batch;
pub mod transaction;
pub mod mktree;
pub mod account;
use std::collections::HashMap;
use std::fmt;


impl fmt::Display for Batch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pre-State root: {} \nPost-State root: {}\n ###Transactions###\n{}", self.pre_state_root, self.post_state_root, self.transactions)
    }
}

impl fmt::Display for Transactions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tx_display = String::from("====================\n");
        for tx in self.b_transactions.iter() {
            tx_display.push_str(format!("From: {}\nTo: {}\nValue: {}\n---\n", tx.from, tx.to, tx.value).as_str());
        }
        tx_display.push_str("====================\n");
        write!(f, "{}", tx_display)
    }
}

pub struct Batch {
    pre_state_root: String,
    post_state_root: String,
    transactions: Transactions,
}

pub struct Accounts {
    hash_map: HashMap<u32, account::Account>,
}

pub struct Transactions {
    b_transactions: Vec<transaction::Transaction>,
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
        for (_, acc) in &self.hash_map {
            leafs[acc.id as usize] = *acc.hash().as_bytes();
        }
        mktree::root2(leafs)
    }
}

impl Transactions {
    pub fn new() -> Transactions {
        Transactions {b_transactions: Vec::new()}
    }
    pub fn add(&mut self, tx: transaction::Transaction) {
        self.b_transactions.push(tx);
    }
}

impl Batch {
    pub fn new(acc: &mut Accounts, txs: Transactions) -> Batch {
        let tmp_pre = acc.merkle_tree();
        for tx in txs.b_transactions.iter() {
            if tx.value > 0 {
                let ok: bool = false;
                match acc.hash_map.get_mut(&tx.from) {
                    Some(sender) => {
                        if tx.value < sender.balance {
                            sender.balance -= tx.value;
                        }
                        else {
                            println!("Sender {} does not have enough funds!", tx.from)
                        }
                    }
                    None => println!("Sender {} not found!", tx.from)
                }
                if ok {
                    match acc.hash_map.get_mut(&tx.to) {
                        Some(receiver) => {
                            receiver.balance += tx.value;
                        },
                        None => println!("{} Account not found!", tx.to)
                    }
                }
                
            }
        }
        Batch {
            post_state_root: acc.merkle_tree(),
            pre_state_root: tmp_pre,
            transactions: txs
        }
    }
}
