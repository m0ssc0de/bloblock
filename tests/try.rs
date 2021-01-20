use bloblock::blob;
use std::env;

#[test]
fn haha() {
    let account = env::var("STORAGE_ACCOUNT").expect("failed read STORAGE_ACCOUNT from env");
    let key = env::var("STORAGE_MASTER_KEY").expect("failed read STORAGE_MASTER_KEY from env");
    let _ = blob::Blob::download(&account, &key, "justry3", "test.txt.txt");
    blob::Blob::insert();
    assert_eq!(2, 2);
}
