use std::{collections::LinkedList, vec};

#[test]
fn test_chapter4() {
    let vec = vec!["hello".to_string(), "world".to_string()];
    let i = vec.iter();
    for elem in i {
        println!("string:{elem}");
    }

    let mut l = LinkedList::new();
    l.push_back(100);
    l.push_back(200);
    println!("{:?}", l);
}
