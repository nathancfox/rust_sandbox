use std::io::{self, Write};

fn main() {
    println!("Welcome to the Employee Database Interface.");
    println!("\nValid commands are:");
    println!("    add NAME to DEPT : Add an employee to a department.");
    println!("    show DEPT        : Show all employees in a department.");
    println!("    quit             : Quit this session.\n");
    loop {
        let command = get_sentence();
        let command_length = command.chars().count();
        if command_length >= 4 {
            if &command[..4] == "quit" {
                break
            }
        }
    }
}

fn get_sentence() -> String {
    let sentence = loop {
        print!(">>> ");
        io::stdout().flush();
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
