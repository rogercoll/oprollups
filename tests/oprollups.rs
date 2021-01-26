use oprollups as opr;
use oprollups::batch;
use oprollups::transaction;
use oprollups::mktree;
use oprollups::account;
use hex;

#[test]
fn test_hello() {
    assert_eq!(opr::hello(), "Hello");
}

#[test]
fn test_hello_batch() {
    assert_eq!(batch::hello_batch(), "Hello_Batch");
}

//Testing transaction module
#[test]
fn test_transaction() {
    assert_eq!(transaction::new(3,4,5).value, 5)
}

#[test]
fn test_sign_transaction() {
    let mut t = transaction::new(1,2,3);
    t.sign();
    assert_eq!(t.signature, 3)
}

//Testing Merkle Tree

#[test]
fn test_merkle_tree_root() {
    assert_eq!(mktree::root(), "8510B9169656743755BB8580A17E93AE16CC0C07699F053554C541DD5AA7F3B7".to_string())
}

#[test]
fn hash_person() {    
    let t1 = account::Person{id: 1, value:1};
    assert_eq!("e14741cbd9ec785db9faca2d4b201badecf2b3c539bfba7092d623df7bea7174" , &t1.hash().to_hex()[..]);
    let t2 = account::Person{id: 1, value:2};
    assert_eq!("9a7092de278ac177df01118bfbce5b4af26d3e998e19141a5331c9c960793027" , &t2.hash().to_hex()[..]);
    let t3 = account::Person{id: 2, value:1};
    assert_eq!("3e95d124e870f127ae8660d6523f85f3ff21ab7acfae66c9dd87d4b771e83fb2" , &t3.hash().to_hex()[..])
}