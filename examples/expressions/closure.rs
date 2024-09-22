// https://doc.rust-lang.org/reference/expressions/closure-expr.html

fn ten_times<F>(f: F)
where
    F: Fn(i32),
{
    for index in 0..10 {
        f(index);
    }
}

fn main() {
    ten_times(|j| println!("hello, {}", j));
    // With type annotations
    ten_times(|j: i32| -> () { println!("hello, {}", j) });

    let word = "konnichiwa".to_owned();
    ten_times(move |j| println!("{}, {}", word, j));
}
