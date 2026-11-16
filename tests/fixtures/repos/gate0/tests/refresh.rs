use gate0::store::TokenStore;

#[test]
fn refreshes_known_token() {
    let mut store = TokenStore::new();
    store.insert("abc");
    assert!(gate0::refresh(&store, "abc").is_some());
}
