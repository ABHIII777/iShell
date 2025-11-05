use std::io::{self, Write};
use std::process::Command;
use std::env;

fn current_directory() {
    match env::current_dir() {
        Ok(path) => println!("The current working directory is {}", path.display()),
        Err(e) => eprintln!("The error occured is: {}", e),
    }
}

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
