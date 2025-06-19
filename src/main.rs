use std::io;
use std::io::Write;
use std::fs::File;
use indoc::indoc;
use colored::Colorize;

const ADD_ITEM: u8 = 1;
const DELETE_ITEM: u8 = 2;

fn print_instructions() {
    let art = indoc!{r" _____  _    ____  _  ______  _
                      |_   _|/ \  / ___|| |/ /  _ \| |
                        | | / _ \ \___ \| ' /| |_) | |
                        | |/ ___ \ ___) | . \|  _ <|_|
                        |_/_/   \_\____/|_|\_\_| \_(_)"};
    println!( "{}", art );
    println!("Welcome to Taskr!");
    println!("\n 1 - Add a new task\n 2 - Delete a task\n");
}

fn print_tasks(tasks: &Vec<String>) {
    println!("\n\nCurrent Tasks:");

    let mut counter: u8 = 1;
    for task in tasks {
        print!("{counter}. {task}");
        counter += 1;
    }

    println!("\n 1 - Add a new task\n 2 - Delete a task\n");
}

fn write_to_file(file_name: String, tasks: &Vec<String>) {
    let mut file = File::create(&file_name).expect("Failed to create file");

    for task in tasks {
        let result = file.write_all(task.as_bytes());
        match result {
            Ok(var) => var,
            Err(error) => panic!("Error encountered while writing to file: {}", error),
        };
    }

    println!("Tasks have been saved in {}.", &file_name);
}

fn main() {
    print_instructions();

    let success = "Successfully added task!".green();
    
    let mut tasks: Vec<String> = vec![]; // Tasks that can be added to/deleted from

    loop {

        let mut action: String = String::new();
        
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read option\n");

        let action: u8 = match action.trim().parse() {
            Ok(arg) => arg,
            Err(_) => continue,
        };

        if action == ADD_ITEM {
            println!("Name of new task: ");

            let mut new_task: String = String::new();
            io::stdin()
                .read_line(&mut new_task)
                .expect("Failed to read option\n");

            tasks.push(new_task);

            println!("{success}");
        } else if action == DELETE_ITEM {
            println!("Enter the index of the task to delete");

            let mut index_str: String = String::new();
            io::stdin()
                .read_line(&mut index_str)
                .expect("Failed to read option\n");

            let idx: usize = match index_str.trim().parse() {
                Ok(arg) => arg,
                Err(_) => continue,
            };

            if idx > tasks.len() {
                let failure = "Attempted to delete non-existent task!".red();
                panic!("{}", failure);
            }

            tasks.remove(idx - 1);
        } else {
            let invalid_action = "Invalid Action".red();
            println!("{}: {}", invalid_action, action);
            break;
        }
        print_tasks(&tasks);

        write_to_file("tasks.txt".to_string(), &tasks);
        // Need to close file after we are done with it
        // Need a way to open task file if one already exists
    }
}
