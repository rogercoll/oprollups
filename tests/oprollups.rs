use oprollups as opr;
use oprollups::batch;
use oprollups::transaction;
use oprollups::mktree;
use oprollups::account;


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
    t.sign("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721");
    assert_eq!(hex::encode(t.signature), "021f5925b1955bd4b36286471dc37d141520ee1303423a59faf2e2e1f70d1012")
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
    let t1 = account::new(1, 1);
    //let t1_expected: [u8; 32] = [0; 32];
    let t1str: String = t1.hash().iter()
                               .map(|b| format!("{:02x?}", b))
                               .collect();
    assert_eq!(t1str , t1.hash_str());
    //assert!(t1_expected.iter().zip(t1.hash().iter()).all(|(a,b)| a == b), "Arrays are not equal");
    let t2 = account::new(1, 2);
    let t2str: String = t2.hash().iter()
                               .map(|b| format!("{:02x?}", b))
                               .collect();
    assert_eq!(t2str , t2.hash_str());
    assert_ne!(t1str, t2str);
    let t3 = account::new(2, 1);
    let t3str: String = t3.hash().iter()
                               .map(|b| format!("{:02x?}", b))
                               .collect();
    assert_eq!(t3str , t3.hash_str());
    assert_ne!(t1str, t3str);
    assert_ne!(t2str, t3str);

    let t4 = account::new(2, 2);
    let t4str: String = t4.hash().iter()
                                .map(|b| format!("{:02x?}", b))
                                .collect();
    assert_eq!(t4str , t4.hash_str());
    assert_ne!(t1str, t4str);
    assert_ne!(t2str, t4str);
    assert_ne!(t3str, t4str);
}

//Testing lib
#[test]
fn test_new_merkle_tree() {
    let a1 = account::new(1002, 1);
    let a2 = account::new(12, 2);
    let a3 = account::new(345, 1);
    let a4 = account::new(999, 2);
    let mut all_accounts = opr::Accounts::new();
    all_accounts.add(a1);
    all_accounts.add(a2);
    all_accounts.add(a3);
    all_accounts.add(a4);
    println!("Merkle:  {}", all_accounts.merkle_tree());
    //println!("Merkle:  {}", a1.balance);

}

#[test]
fn test_new_batch() {
    let mut all_accounts = opr::Accounts::new();
    all_accounts.add(account::new(1002, 100));
    all_accounts.add(account::new(12, 100));
    all_accounts.add(account::new(345, 100));
    all_accounts.add(account::new(999, 100));
    all_accounts.add(account::new(1022, 100));
    all_accounts.add(account::new(1234, 100));
    all_accounts.add(account::new(34, 100));
    all_accounts.add(account::new(997, 100));
    let t1 = transaction::new(0,2,5);
    let t2 = transaction::new(0,2,82);
    let t3 = transaction::new(7,1,4);

    let mut txs = opr::Transactions::new();
    txs.add(t1);
    txs.add(t2);
    txs.add(t3);
    let b1 = opr::Batch::new(&mut all_accounts, txs);
    println!("{}", b1)
}