use std::io;
use std::io::Write;
use std::process;
use std::fs;
use std::path::Path;


fn main() {
    welcome();
    ensure_journal_directory();
    run_main_menu();
}

fn run_main_menu() {
    main_menu_options();
    loop {
        print!(">>> ");
        match io::stdout().flush() {
            Ok(_) => (),
            Err(error) => {
                println!("Failed to flush stdout:\n{}", error);
                process::exit(1);
            }
        }
        let mut menu_selection = String::new();
        match io::stdin().read_line(&mut menu_selection) {
            Ok(_) => (),
            Err(error) => {
                println!("Failed to read menu_selection:\n{}", error);
                process::exit(1);
            }
        }
        menu_selection.pop();  // Strip trailing newline
        match menu_selection.as_str() {
            "new" => {
                let mut new_entry: Vec<String> = Vec::new();
                println!("");
                get_journal_entry(&mut new_entry);
                let new_entry = format!("{}\n", new_entry.join("\n"));
                match fs::write(".journal/entry.txt", new_entry) {
                    Ok(_) => (),
                    Err(error) => {
                        println!("Failed to write new entry to file:\n{}", error);
                    }
                }
                continue;
            }
            "read" => {
                let entry = match fs::read_to_string(".journal/entry.txt") {
                    Ok(file_contents) => file_contents,
                    Err(error) => {
                        println!("Failed to read entry from file:\n{}", error);
                        "".to_string()
                    }
                };
                println!("\n{}", entry);
                continue;
            }
            "options" => {
                main_menu_options();
                continue;
            }
            "quit" => {
                process::exit(0);
            }
            "exit" => {
                process::exit(0);
            }
            invalid => {
                println!("{} is not a valid menu option!", invalid);
                continue
            }
        }
    }
}

fn get_journal_entry(empty_entry: &mut Vec<String>) {
    loop {
        let mut next_line = String::new();
        match io::stdin().read_line(&mut next_line) {
            Ok(_) => (),
            Err(error) => {
                println!("Failed to read new entry:\n{}", error);
                process::exit(1);
            }
        }
        next_line.pop();  // Strip trailing newline
        match next_line.as_str() {
            "" => {
                return;
            }
            _ => {
                empty_entry.push(next_line);
                continue;
            }
        }
    }
}

fn ensure_journal_directory() {
    if ! Path::new(".journal/").is_dir() {
        match fs::create_dir(".journal/") {
            Ok(_) => (),
            Err(error) => {
                println!("Failed to create .journal/ directory:\n{}", error);
                process::exit(1);
            }
        }
    }
}

fn main_menu_options() {
    println!("");
    println!("    (new)      Write new entry");
    println!("    (read)     Read the last entry");
    println!("    (options)  Display these options");
    println!("    (quit)     Exit program");
    println!("");
}

fn welcome() {
    println!("   _                              _");
    println!("  (_) ___  _   _ _ __ _ __   __ _| |");
    println!("  | |/ _ \\| | | | '__| '_ \\ / _` | |");
    println!("  | | (_) | |_| | |  | | | | (_| | |");
    println!(" _/ |\\___/ \\__,_|_|  |_| |_|\\__,_|_|");
    println!("|__/");
    print!("\n\n");
    println!("Welcome to journal! Here you can write journal entries,");
    println!("and then read them back later!");
}
