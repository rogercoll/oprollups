pub mod batch;
pub mod transaction;
pub mod mktree;
pub mod account;

pub fn hello() -> String {
    return "Hello".to_string()
}

pub fn new_merkle_tree() -> String {
    let a1 = account::Account{id: 1, value: 2};
    let a2 = account::Account{id: 1, value: 2};
    let a3 = account::Account{id: 2, value: 2};
    let a4 = account::Account{id: 1, value: 2};
    mktree::root(a1.hash().as_bytes(),a2.hash().as_bytes(),a3.hash().as_bytes(),a4.hash().as_bytes())
}