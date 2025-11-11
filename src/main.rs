use std::io::{self, Write};
use std::process::Command;
use std::env;
use tabled::{builder::Builder};
use std::fs;
use colored::*;
use std::os::unix::fs::MetadataExt;
use users::*;

fn current_directory() {
    match env::current_dir() {
        Ok(path) => println!("The current working directory is {}", path.display()),
        Err(e) => eprintln!("The error occured is: {}", e),
    }
}

fn mode_format(num: &u32) -> String {
    let mut str = String::new();

    str.push(if num & 0o400 != 0 { 'r' } else { '-' });
    str.push(if num & 0o200 != 0 { 'w' } else { '-' });
    str.push(if num & 0o100 != 0 { 'x' } else { '-' });

    str.push(if num & 0o040 != 0 { 'r' } else { '-' });
    str.push(if num & 0o020 != 0 { 'w' } else { '-' });
    str.push(if num & 0o010 != 0 { 'x' } else { '-' });

    str.push(if num & 0o004 != 0 { 'r' } else { '-' });
    str.push(if num & 0o002 != 0 { 'w' } else { '-' });
    str.push(if num & 0o001 != 0 { 'x' } else { '-' });

    str
}

fn lsl_command() {
    let mut builder = Builder::new();

    builder.push_record([
        "Name".cyan().bold().to_string(),
        "Type".cyan().bold().to_string(),
        "Target".cyan().bold().to_string(),
        "Read Only".cyan().bold().to_string(),
        "Mode".cyan().bold().to_string(),
        "Num Links".cyan().bold().to_string(),
        "Inode".cyan().bold().to_string(),
        "User".cyan().bold().to_string(),
        "Group".cyan().bold().to_string(),
        "Size".cyan().bold().to_string(),
    ]);

    for entry in fs::read_dir(".").unwrap() {
        let entries = entry.unwrap();
        let meta = entries.metadata().unwrap();

        let filename = entries.file_name().to_str().unwrap().to_string();
        let size = meta.len();
        let file_type = if meta.is_dir() { "Dir" } else { "File" };

        let inode = meta.ino();
        let uid = meta.uid();
        let gid = meta.gid();
        
        let num_links = meta.nlink();
        // let mode = meta.mode().to_string();
        let mode = mode_format(&meta.mode());

        let read_only = meta.permissions().readonly();


        let user = get_user_by_uid(uid)
            .map(|u| u.name().to_string_lossy().into_owned())
            .unwrap_or("unknown".into());

        let group = get_group_by_gid(gid)
            .map(|g| g.name().to_string_lossy().into_owned())
            .unwrap_or("unknown".into());

        let main_file_name = if meta.is_dir() {
            filename.green().bold().to_string()
        } else {
            filename.white().to_string()
        };

        let mut target = String::new();
        if meta.file_type().is_symlink() {
            target = fs::read_link(entries.path())
                .unwrap()
                .to_string_lossy()
                .into_owned();
        }

        builder.push_record([main_file_name, file_type.to_string(), target, read_only.to_string(), mode.to_string(), num_links.to_string(), inode.to_string(), user, group, size.to_string()]);
    }

    let mut table = builder.build();
    let main_table = table.with(tabled::settings::Style::modern()).with(tabled::settings::Alignment::left());

    println!("{}", main_table);
}


fn lsa_command() {
    let mut builder = Builder::default();

    builder.push_record([
        "Name".cyan().bold().to_string(),
        "Type".cyan().bold().to_string(),
        "Size".cyan().bold().to_string(),
    ]);

    for entry in fs::read_dir(".").unwrap() {
        let entries = entry.unwrap();
        let meta = entries.metadata().unwrap();

        let size = meta.len().to_string();
        let file_type = if meta.is_dir() { "Dir" } else { "File" };

        let filename = entries.file_name().to_str().unwrap().to_string();
        
        let main_file_name = if meta.is_dir() {
            filename.green().bold().to_string()
        } else {
            filename.white().to_string()
        };

        builder.push_record([main_file_name, file_type.to_string(), size]);
    }

    let mut table = builder.build();
    let main_table = table.with(tabled::settings::Style::modern()).with(tabled::settings::Alignment::left());

    println!("{}", main_table);
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

        let main_file_name = if meta.is_dir() {
            filename.green().bold().to_string()
        } else {
            filename.white().to_string()
        };

        let size = meta.len().to_string();
        let file_type = if meta.is_dir() { "Dir" } else { "File" };

        builder.push_record([main_file_name, file_type.to_string(), size]);
    }

    let mut table = builder.build();
    let main_table = table.with(tabled::settings::Style::modern()).with(tabled::settings::Alignment::left());

    println!("{}", main_table);
}

fn make_dir(name: &str) {
    match fs::create_dir(name) {
        Ok(_) => println!("A new directory is created named: {}", name),
        Err(e) => eprintln!("An error occured while creating new directory {}", e),
    }
}

fn remove_dir(name: &str) {
    match fs::remove_dir(name) {
        Ok(_) => println!("The directory is removed. "),
        Err(e) => eprintln!("An error occured during removing the directory. {}", e),
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

            "ls" => {
                tabled_directory();
            }

            "ls -a" => {
                lsa_command();
            }

            "ls -l" => {
                lsl_command();
            }

            _ if line.contains("mkdir ") => {
                let dir = line.strip_prefix("mkdir ").unwrap().trim();
                make_dir(dir);
            }

            _ if line.contains("rmdir ") => {
                let dir = line.strip_prefix("rmdir ").unwrap().trim();
                remove_dir(dir);
            }

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
