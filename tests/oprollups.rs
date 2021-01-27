use oprollups as opr;
use oprollups::batch;
use oprollups::transaction;
use oprollups::mktree;
use oprollups::account;

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
    let mut h1: [u8; 32] = [0u8; 32];
    let mut h2: [u8; 32] = [0u8; 32];
    let mut h3: [u8; 32] = [0u8; 32];
    let mut h4: [u8; 32] = [0u8; 32];
    h1[0] = 0x01;
    h2[0] = 0x01;
    h3[0] = 0x01;
    h4[0] = 0x01;

    assert_eq!(mktree::root(&h1, &h2, &h3, &h4), "0BDD69755E1C61796E85C475C1A6C878FC89E3B902E0E384B3EEC51E35E97003".to_string());

    h1[0] = 0x02;
    h2[0] = 0x01;
    h3[0] = 0x01;
    h4[0] = 0x01;

    assert_eq!(mktree::root(&h1, &h2, &h3, &h4), "2B89C1D8A700536C13A0EC918722F07848DC6D25F93804A441A76C666D40E145".to_string());
    
    h1[0] = 0x01;
    h2[0] = 0x02;
    h3[0] = 0x01;
    h4[0] = 0x01;

    assert_eq!(mktree::root(&h1, &h2, &h3, &h4), "FFDE22BFF13493932AC9EFA33B305576EB082425BD2A1505C08B3B76548B07B1".to_string());

    h1[0] = 0x01;
    h2[0] = 0x01;
    h3[0] = 0x02;
    h4[0] = 0x01;

    assert_eq!(mktree::root(&h1, &h2, &h3, &h4), "6D8DDBEE2B18A5634287EA7C59C34013A65DD7D5F6AA8959F40A02F1830F2FDD".to_string());
    h1[0] = 0x01;
    h2[0] = 0x01;
    h3[0] = 0x01;
    h4[0] = 0x02;

    assert_eq!(mktree::root(&h1, &h2, &h3, &h4), "95BEE25B6BD37BD1E44C976D9DD2BBB188026B8993D0A705CE5701268F658556".to_string())
}


//Testing Account

#[test]
fn hash_person() {    
    let t1 = account::new(1, 1, 0);
    assert_eq!("e14741cbd9ec785db9faca2d4b201badecf2b3c539bfba7092d623df7bea7174" , &t1.hash().to_hex()[..]);
    let t2 = account::new(1, 2, 0);
    assert_eq!("9a7092de278ac177df01118bfbce5b4af26d3e998e19141a5331c9c960793027" , &t2.hash().to_hex()[..]);
    let t3 = account::new(2, 1, 0);
    assert_eq!("3e95d124e870f127ae8660d6523f85f3ff21ab7acfae66c9dd87d4b771e83fb2" , &t3.hash().to_hex()[..]);
    let t4 = account::new(2, 2, 0);
    assert_eq!("9bffdbf7084e53304ca5f89dc72ab31c67430a121a68d217e7864c07a2444a0c" , &t4.hash().to_hex()[..])
}

//Testing lib
#[test]
fn test_new_merkle_tree() {
    let a1 = account::new(1002, 1, 0);
    let a2 = account::new(12, 2, 1);
    let a3 = account::new(345, 1, 2);
    let a4 = account::new(999, 2, 3);
    let mut all_accounts = opr::Accounts::new();
    all_accounts.add(a1);
    all_accounts.add(a2);
    all_accounts.add(a3);
    all_accounts.add(a4);
    println!("Merkle:  {}", all_accounts.merkle_tree());
}