use std::io::{self, Write};
use std::collections::HashMap;

fn main() {
    println!("Welcome to the Employee Database Interface.");
    print_valid_commands();
    let mut database = HashMap::new();
    loop {
        let command = get_sentence();
        let split_command: Vec<&str> = command.split(" ").collect();
        if split_command.len() == 0 {
            continue
        }
        match split_command[0] {
            "add" => add_employee(split_command, &mut database),
            "show" => show_department(split_command, &database),
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

fn add_employee(split_command: Vec<&str>, database: &mut HashMap<String, String>) {
    let mut split_index = 0;
    let split_index_ref = &mut split_index;
    for (i, word) in split_command.iter().enumerate() {
        if *word == "to" {
            *split_index_ref = i;
            break
        }
    }
    let name = split_command[1..split_index].join(" ");
    let department = split_command[split_index+1..].join(" ");
    database.insert(department, name);
}

fn show_department(split_command: Vec<&str>, database: &HashMap<String, String>) {
    let department = split_command[1..].join(" ");
    let employee = match database.get(&department) {
        Some(name) => name,
        None => "",
    };
    println!("The employee in this department is {}", employee);
}

fn print_valid_commands() {
    println!("\nValid commands are:");
    println!("    add NAME to DEPT : Add an employee to a department.");
    println!("    show DEPT        : Show all employees in a department.");
    println!("    quit             : Quit this session.\n");
}
