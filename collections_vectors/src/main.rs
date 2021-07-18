use std::io;
use std::collections::HashMap;

fn main() {
    let mut v: Vec<f64> = Vec::new();
    println!("Enter a list of integers, separated by spaces.");
    let mut int_list = String::new();
    io::stdin()
        .read_line(&mut int_list)
        .expect("Failed to read list.");
    for entry in int_list.split(' ') {
       let entry: f64 = match entry.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!(concat!(
                        "Invalid input. Every element must be ",
                         "an integer, and they must be separated ",
                         "by single spaces."));
                return;
            },
        };
        v.push(entry);
    }
    println!("Mean   : {:.2}", mean(&v));
    println!("Median : {}", median(&v));
    println!("Mode   : {}", mode(&v));
    println!("   **Note - If there are multiple modes, it will be a single value that is not deterministic.");
}

fn mean(list: &Vec<f64>) -> f64 {
    let mut sum: f64 = 0.0;
    for num in list {
        sum += num;
    }
    let length = list.len() as f64;
    return sum / length;
}

fn median(list: &Vec<f64>) -> f64 {
    let length = list.len() as u32;
    let middle = ((length as f64 / 2.0).floor() - 1.0) as isize;
    let extra: isize = 1;
    if length % 2 == 0 {
        return (list[middle as usize] + list[(middle + extra) as usize]) / 2.0;
    } else {
        return list[(middle + extra) as usize];
    }
}

fn mode(list: &Vec<f64>) -> f64 {
    let mut values = HashMap::new();
    for num in list {
        let count = values.entry(num.to_string()).or_insert(1);
        *count += 1;
    }
    let mut mode_value = String::new();
    let mut count = 0;
    for (key, value) in &values {
        if *value > count {
            mode_value = key.to_string();
            count = *value;
        }
    }
    return mode_value.parse().expect("mode tried to return a non-numeric");
}
