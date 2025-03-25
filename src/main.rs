use colored::Colorize;
use hostname;
use std::env;
use std::io::{self, Write};
use std::process::{Command, Stdio};

fn prompt() {
    let user_name = env::var("USER").unwrap().green();
    let user_hostname = hostname::get().unwrap().to_str().unwrap().to_owned();
    let home_dir = env::var("HOME").unwrap();
    let current_dir = env::current_dir().unwrap();
    let display_dir = current_dir.display().to_string().replace(&home_dir, "~");
    print!(
        "{}@{} {}{}",
        user_name,
        user_hostname,
        display_dir,
        ">".green()
    );
    io::stdout().flush().unwrap();
}

fn main() {
    loop {
        prompt();

        // Read the input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // Handle built-in commands
        if input == "exit" {
            break;
        } else if input.starts_with("cd ") {
            let dir = &input[3..];
            if let Err(err) = env::set_current_dir(dir) {
                eprintln!("cd: {}", err);
            }
            continue;
        }

        // Split the input into command and arguments
        let mut parts = input.split_whitespace();
        if let Some(command) = parts.next() {
            let args = parts.collect::<Vec<&str>>();

            // Execute the command
            let child = Command::new(command)
                .args(&args)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn();

            match child {
                Ok(mut child) => {
                    child.wait().unwrap();
                }
                Err(err) => eprintln!("Error executing command: {}", err),
            }
        }
    }
}
