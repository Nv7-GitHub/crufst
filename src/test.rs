use super::*;

#[test]
fn test_btree() {
    let mut tree = BTree::new();
    for i in 0..10 {
        tree.insert(i);
        println!("{tree:?}\n");
    }
}