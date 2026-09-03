fn add(first_value: i32, second_value: i32) -> i32 {
    first_value + second_value
}

fn print<T: std::fmt::Display>(to_print: T) {
    println!("{}", to_print);
}

fn main() {
    let a: i32 = 5;
    let b: i32 = 5;

    let c: i32 = add(a, b);

    print(c);
    print(5);
}
