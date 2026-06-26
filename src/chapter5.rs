use std::thread::current;

macro_rules! curried_add {
    ($T:ty, $a:expr) => {{
        struct Closure;

        impl Closure {
            fn call(b: $T) -> $T {
                const A: $T = $a;
                A + b
            }
        }

        Closure::call
    }};
}

#[test]
fn test_chapter5() {
    for ch in "你好hello world".chars() {
        println!("{}", ch);
    }

    let re = regex::Regex::new(r"\\s+").unwrap();
    let result = re.split("你好hello world").collect::<Vec<_>>();
    result.iter().for_each(|item| println!("{}", item));

    let add7 = curried_add!(i32, 7);
    println!("{}", add7(10));
}
