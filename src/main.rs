use std::io;
fn main() {
    let n = get_input();
    let result = fibonacci(n);
    println!("The {n}th Fibonacci number is: {result}");
}

fn get_input() -> u32 {
    println!("Enter the value of n:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: u32 = input.trim().parse().expect("Please enter a valid number");
    n
}

fn fibonacci(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp + b;
    }

    a
}
