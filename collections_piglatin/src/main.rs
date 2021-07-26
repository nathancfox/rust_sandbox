use std::io;

struct Letters {
    vowels: [char; 12],
    word_bounds: [char; 16],
}

#[derive(Debug)]
enum Element {
    Word(String),
    Separator(String),
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
                words.push(Element::Word(word));
            }
            words.push(Element::Separator(c.to_string()));
            word = "".to_string();
        } else if i == sentence.len() - 1 {
            word.push(c);
            if word_length != 0 {
                words.push(Element::Word(word))
            }
            word = "".to_string();
        } else {
            word.push(c);
        };
    }
    let mut output_sentence = String::new();
    for word in &words {
        let new_word = match word {
            Element::Word(string) => piglatin_word(string),
            Element::Separator(string) => string.to_string(),
        };
        // println!("{:?}", new_word);
        output_sentence.push_str(&new_word);
    }
    println!("{}", output_sentence);
}

fn get_letters() -> Letters {
    let letters = Letters {
        vowels: ['a', 'e', 'i', 'o', 'u', 'y',
                 'A', 'E', 'I', 'O', 'U', 'Y'],
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
            Ok(_) => (),
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
