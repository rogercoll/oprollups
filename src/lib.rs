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
    next_id: u32,
    map_id: HashMap<u32, account::Account>,
    //Hash to seach the id of a giving address
    map_addr: HashMap<String, u32>,
}

pub struct Transactions {
    b_transactions: Vec<transaction::Transaction>,
}

impl Accounts {
    pub fn new() -> Accounts {
        Accounts {next_id:0, map_id: HashMap::new(), map_addr: HashMap::new()}
    }
    pub fn add(&mut self, mut user: account::Account) {
        user.set_id(self.next_id);
        let hash_addr = user.hash_addr();
        self.map_id.insert(self.next_id, user);
        self.map_addr.insert(hash_addr, self.next_id);
        self.next_id += 1;
    }
    pub fn merkle_tree(&self) -> String {
        let mut leafs:Vec<[u8;32]> = vec![[0u8;32]; self.map_id.len()];    
        for (_, acc) in &self.map_id {
            leafs[acc.get_id() as usize] = acc.hash();
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
                match acc.map_id.get_mut(&tx.from) {
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
                    match acc.map_id.get_mut(&tx.to) {
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
