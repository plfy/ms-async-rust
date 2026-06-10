use std::{
    cell::Cell,
    ops::Deref,
    sync::{Arc, Mutex},
};

#[test]
fn test_chapter11() {
    let num: i32 = 200;
    let arc = Arc::new(Mutex::new(num));

    let a1 = arc.clone();
    let a2 = arc.clone();

    *a1.lock().unwrap() += 1;

    println!("{:?},{:?}", a1, a2);
}

fn hello(s: &str) {
    println!("{}", s);
}

#[test]
fn test_deref() {
    let name = String::from("Rust");
    let _ = hello(&name);
    let _ = hello(Deref::deref(&name)); // 翻译上一行的代码
}

#[test]
fn test_mut() {
    let mut num: i32 = 100;
    let m = &mut num;
    *m += 1;
    println!("num is:{}", num);
}

struct Change {
    id: i32,
    s: Cell<String>,
}

#[test]
fn test_cell() {
    let c = Change {
        id: 100,
        s: Cell::new("hello world".to_string()),
    };
    c.s.replace("hello world!".to_string());
    c.s.replace("world!".to_string());
    println!("{:?}", c.s.take());
}

#[test]
fn test_closure() {
    let name = "hello world".to_string();
    println!("{}", std::mem::size_of_val(&name));
    let f = || {
        let n = &name;
        println!("{}", std::mem::size_of_val(&n));
    };
    f();
    println!("{}", name);
}

#[tokio::test]
async fn test_box() {
    let mut num = Box::new(100);
    *num += 100;
    println!("{}", num);

    let f = async {
        println!("hello world");
    };
    let _ = f.await;
}
