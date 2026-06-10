use std::time::Duration;

use tokio::time::sleep;

mod chapter1;
mod chapter11;
mod chapter2;
mod chapter3;
mod chapter4;
mod chapter5;

async fn print_msg(f: impl AsyncFn(String), msg: String) {
    f(msg).await;
    sleep(Duration::from_secs(2)).await;
}

#[tokio::main]
async fn main() {
    print_msg(
        async move |s: String| {
            println!("msg is:{}, len is:{}", s, s.len());
        },
        "hello world".to_string(),
    )
    .await;
    println!("end main");

    let f = async {
        let _ = sleep(Duration::from_secs(1)).await;
        100
    };
    println!("result:{}", f.await);

    let s: &'static str = "hello world";
    println!("{}", s);
}
