use std::io;

fn main() {
    let mut n_fibo = String::new();
    let mut n_fibo_num: u32 = 0;
    loop {
        println!("Enter a positive number. That Fibonacci number will be printed.");
        io::stdin()
            .read_line(&mut n_fibo)
            .expect("Failed to read line!");
        n_fibo = n_fibo.trim().to_string();
        n_fibo_num = match n_fibo.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break;
    }
    let result = fibo(n_fibo_num);
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
    let mut temp: u64 = 0;
    for step in 2..n {
        temp = two_back + one_back;
        two_back = one_back;
        one_back = temp;
        result = temp;
    }
    result
}
