use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::env;

fn main() {
    loop {
        // Display the prompt
        print!("rust-shell> ");
        io::stdout().flush().unwrap();

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
            let mut child = Command::new(command)
                .args(&args)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn();

            match child {
                Ok(mut child) => { child.wait().unwrap(); },
                Err(err) => eprintln!("Error executing command: {}", err),
            }
        }
    }
}