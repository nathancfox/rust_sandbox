use std::io;

struct Letters {
    vowels: [char; 6],
    consonants: [char; 20],
    word_bounds: [char; 16],
}

fn main() {
    let letters = get_letters();
    let sentence = get_sentence();
    let mut words = Vec::new();
    let mut word = String::new();
    for (i, c) in sentence.chars().enumerate() {
        let word_length = word.len();
        if letters.word_bounds.contains(&c) {
            if word_length != 0 {
                words.push(word)
            }
            word = "".to_string();
        } else if i == sentence.len() - 1 {
            word.push(c);
            if word_length != 0 {
                words.push(word)
            }
            word = "".to_string();
        } else {
            word.push(c);
        };
    }
    let mut output_sentence = String::new();
    for word in &words {
        let new_word = piglatin_word(word);
        output_sentence.push_str(&new_word);
        output_sentence.push_str(" ");
    }
    println!("{}", output_sentence);
}

fn get_letters() -> Letters {
    let letters = Letters {
        vowels: ['a', 'e', 'i', 'o', 'u', 'y'],
        consonants: ['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k',
                     'l', 'm', 'n', 'p', 'q', 'r', 's', 't',
                     'v', 'w', 'x', 'z'],
        word_bounds: [' ', '\t', '\n', '.', ',', '?', '!', ':', ';',
                      '"', '\'', '/', '(', ')', '|', '\\'],
    };
    return letters;
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

fn piglatin_word(word: &String) -> String { 
    let letters = get_letters();
    let mut new_word = String::new();
    for (i, c) in word.chars().enumerate() {
        if letters.vowels.contains(&c) {
            if i == 0 {
                new_word.push_str(word);
                new_word.push_str("-hay");
                return new_word;
                // println!("{}-hay", word);
                // break;
            } else {
                new_word.push_str(&word[i..]);
                new_word.push_str("-");
                new_word.push_str(&word[..i]);
                new_word.push_str("ay");
                return new_word;
                // println!("{}", new_word);
                // break;
            }
        }
    }
    return "".to_string();
}
