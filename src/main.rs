use std::io::{self, Write};
use std::process::Command;
use std::env;
// use prettytable::{Table, Cell, Row};
use tabled::{builder::Builder};
use std::fs;
use colored::*;

fn current_directory() {
    match env::current_dir() {
        Ok(path) => println!("The current working directory is {}", path.display()),
        Err(e) => eprintln!("The error occured is: {}", e),
    }
}

fn tabled_directory() {
    let mut builder = Builder::default();

    builder.push_record([
        "Name".cyan().bold().to_string(),
        "Type".cyan().bold().to_string(),
        "Size".cyan().bold().to_string(),
    ]);

    for entry in fs::read_dir(".").unwrap() {
        let entries = entry.unwrap();
        let meta = entries.metadata().unwrap();

        let filename = entries.file_name().to_str().unwrap().to_string();
        let size = meta.len().to_string();
        let file_type = if meta.is_dir() { "Dir" } else { "File" };

        builder.push_record([filename, file_type.to_string(), size]);
    }

    // let table = builder.build().with(tabled::settings::Style::modern()).with(tabled::settings::Alignment::left());
    let mut table = builder.build();
    let main_table = table.with(tabled::settings::Style::modern()).with(tabled::settings::Alignment::left());

    println!("{}", main_table);
}

// fn print_directorytable() {
//     let mut table = Table::new();

//     table.set_format(*prettytable::format::consts::FORMAT_BOX_CHARS);

//     table.add_row(Row::new(vec![
//         Cell::new(&"Name").style_spec("bFb"),
//         Cell::new(&"Type").style_spec("bFb"),
//         Cell::new(&"Size").style_spec("bFb"),
//     ]));
    
//     for entry in fs::read_dir(".").unwrap() {
//         let entries = entry.unwrap();
//         let meta = entries.metadata().unwrap();

//         let filename = entries.file_name().to_str().unwrap().to_string();

//         let mut name_cell = Cell::new(&filename);

//         if meta.is_dir() { 
//             name_cell = name_cell.style_spec("bFg");
//         } else {
//             name_cell = name_cell.style_spec("bFw");
//         };

//         let size = meta.len();
//         let file_type = if meta.is_dir() { "Dir" } else { "File" };

//         table.add_row(Row::new(vec![
//             name_cell,
//             Cell::new(file_type),
//             Cell::new(&size.to_string()),
//         ]));
//     }

//     table.printstd();
// }

fn main() {
    Command::new("clear").status();
    loop {
        print!("->");
        io::stdout().flush().unwrap();
        let mut command = String::new();

        io::stdin().read_line(&mut command).unwrap();
        let line = command.trim();

        match line {
            "exit" | "quit" => break,
            "ping" => println!("pong"),
            "dir -c" => {
                current_directory();
            }

            "ls" => {
                tabled_directory();
            }

            // "ls" | "dir" => {
            //     print_directorytable();
            // }

            _ if line.contains("cd ") => {
                let dir = line.strip_prefix("cd ").unwrap().trim();
                env::set_current_dir(dir);
            }

            _ if line.contains("change directory to ") => {
                let dir = line.strip_prefix("change directory to ").unwrap().trim();
                env::set_current_dir(dir);
            }
            "clear" => {
                let _ = Command::new("clear").status().unwrap();
            }
            
            _ if !line.is_empty() => {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let cmd = parts[0];
                let args = &parts[1..];
                match Command::new(cmd).args(args).status() {
                    Ok(status) => println!("Process exited with: {}", status),
                    Err(e) => println!("Failed to execute command: {}", e),
                }
            }
            
            _ => { }
        }
    }
}
