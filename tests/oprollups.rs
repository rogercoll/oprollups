use oprollups as opr;
use oprollups::batch;
use oprollups::transaction;
use oprollups::mktree;


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
