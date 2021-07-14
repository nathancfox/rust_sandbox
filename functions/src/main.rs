use std::io;

fn main() {
    let n_fibo: u32 = loop {
        println!("Enter a positive number. That Fibonacci number will be printed.");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        let input = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break input;
    };
    let result = fibo(n_fibo);
    println!("The {}th Fibonacci number is {}", n_fibo, result);
}

fn fibo(n: u32) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else if n == 2 {
        return 1;
    }
    let first = 1;
    let second = 1;
    let mut two_back: u64 = first;
    let mut one_back: u64 = second;
    let mut result: u64 = 0;
    for _ in 2..n {
        let temp: u64 = two_back + one_back;
        two_back = one_back;
        one_back = temp;
        result = temp;
    }
    result
}
