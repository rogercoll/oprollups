use oprollups as opr;

use oprollups::batch;

#[test]
fn test_hello() {
    assert_eq!(opr::hello(), "Hello");
}

#[test]
fn test_helloBatch() {
    assert_eq!(batch::hello_Batch(), "Hello_Batch");
}
