use std::io;

fn main() {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let consonants = ['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k',
                      'l', 'm', 'n', 'p', 'q', 'r', 's', 't',
                      'v', 'w', 'x', 'y', 'z'];
    let word_bounds = [' ', '\t', '\n', '.', ',', '?', '!', ':', ';',
                       '"', '\'', '/', '(', ')', '|', '\\'];
    let sentence = get_sentence();
    println!("Here are the individual words:");
    let mut word = String::new();
    for c in sentence.chars() {
        if word_bounds.contains(&c) {
            if word.len() != 0 {
                println!("{}", word);
            }
            word = "".to_string();
        } else {
            word.push(c);
        };
    }
}

fn get_sentence() -> String {
    let sentence = loop {
        println!("Enter your sentence.");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(bytes) => (),
            Err(_) => {
                println!("Something went wrong. Try again.");
                continue;
            }
        }
        let input = input.trim().to_string();
        break input;
    };
    return sentence;
}
