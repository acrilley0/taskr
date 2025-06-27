use colored::Colorize;
use indoc::indoc;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::BufRead;
use std::io::Result;
use std::io::Write;

const ADD_ITEM: u8 = 1;
const DELETE_ITEM: u8 = 2;

fn print_instructions() {
    let art = indoc! {r" _____  _    ____  _  ______  _
                      |_   _|/ \  / ___|| |/ /  _ \| |
                        | | / _ \ \___ \| ' /| |_) | |
                        | |/ ___ \ ___) | . \|  _ <|_|
                        |_/_/   \_\____/|_|\_\_| \_(_)"};
    println!("{}", art);
    println!("Welcome to Taskr!");
}

fn print_tasks(tasks: &Vec<String>) {
    println!("\n\nCurrent Tasks:");

    let mut counter: u8 = 1;
    for task in tasks {
        println!("{counter}. {task}");
        counter += 1;
    }

    println!("\n 1 - Add a new task\n 2 - Delete a task\n");
}

fn write_to_file(file_name: String, tasks: &Vec<String>) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_name)?;

    for task in tasks.iter() {
        file.write_fmt(format_args!("{}\n", task.trim()))?;
    }

    Ok(())
}

fn read_file(tasks: &mut Vec<String>) -> Result<()> {
    let file = match File::open("./tasks.txt") {
        Err(_) => {
            File::create("./tasks.txt")?;
            return Ok(()); // We don't want to try to read from the file if we just created it
        }
        Ok(file) => file,
    };

    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => tasks.push(line),
            Err(error) => panic!("Can't read line: {}", error),
        };
    }

    Ok(())
}

fn main() -> Result<()> {
    let mut tasks: Vec<String> = vec![]; // Tasks that can be added to/deleted from

    match read_file(&mut tasks) {
        Ok(arg) => arg,
        Err(_) => panic!("FAILURE"),
    };

    print_instructions();

    let success = "Successfully added task!".green();

    loop {
        print_tasks(&tasks);
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

            let err_msg = "WARNING: Not a valid task!".red();
            match tasks.get(idx - 1) {
                Some(x) => println!("Removing task {x}"),
                None => {
                    println!("{err_msg}");
                    print_tasks(&tasks);
                    continue;
                }
            };
            tasks.remove(idx - 1);
        } else {
            let invalid_action = "Invalid Action".red();
            println!("{}: {}", invalid_action, action);
            print_tasks(&tasks);
            continue;
        }

        write_to_file("tasks.txt".to_string(), &tasks)?;
    }
}
