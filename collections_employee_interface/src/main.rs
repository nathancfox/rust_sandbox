use std::io::{self, Write};

fn main() {
    println!("Welcome to the Employee Database Interface.");
    print_valid_commands();
    loop {
        let command = get_sentence();
        let split_command: Vec<&str> = command.split(" ").collect();
        if split_command.len() == 0 {
            continue
        }
        match split_command[0] {
            "add" => add_employee(split_command),
            "show" => show_department(split_command),
            "commands" => print_valid_commands(),
            "quit" => break,
            _ => {
                println!("Not a valid command. Enter \"commands\" to see a list of valid commands.");
            },
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

fn add_employee(split_command: Vec<&str>) {
    println!("Adding employee with command \"{}\"", split_command.join(" "));
}

fn show_department(split_command: Vec<&str>) {
    println!("Showing employees of department with command \"{}\"", split_command.join(" "));
}

fn print_valid_commands() {
    println!("\nValid commands are:");
    println!("    add NAME to DEPT : Add an employee to a department.");
    println!("    show DEPT        : Show all employees in a department.");
    println!("    quit             : Quit this session.\n");
}
