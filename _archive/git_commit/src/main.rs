use chrono::Local;
use std::process::{exit, Command};

fn set_git_config(key: &str, value: &str) -> Result<(), String> {
    let result = Command::new("git")
        .arg("config")
        .arg("--global")
        .arg(key)
        .arg(value)
        .output();

    match result {
        Ok(output) => {
            if output.status.success() {
                println!("Git {} set to: {}", key, value);
                Ok(())
            } else {
                Err(format!("Failed to set Git {}: {}", key, value))
            }
        }
        Err(error) => Err(format!("Error while setting Git {}: {}", key, error)),
    }
}

fn run_git_command(command: &str, args: &[&str]) -> Result<(), String> {
    let result = Command::new("git").arg(command).args(args).output();

    match result {
        Ok(output) => {
            if output.status.success() {
                Ok(())
            } else {
                Err(format!(
                    "Git command failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ))
            }
        }
        Err(error) => Err(format!("Error executing Git command: {}", error)),
    }
}

fn main() {
    let git_email = "mamezni@gmail.com";
    let git_name = "Mezni";

    // Set the global user email and name
    if let Err(error) = set_git_config("user.email", git_email) {
        eprintln!("{}", error);
        exit(1);
    }

    if let Err(error) = set_git_config("user.name", git_name) {
        eprintln!("{}", error);
        exit(1);
    }

    // Stage all changes with `git add -A`
    if let Err(error) = run_git_command("add", &["-A"]) {
        eprintln!("{}", error);
        exit(1);
    }
    println!("Staged all changes with `git add -A`.");

    // Generate commit message with the current date
    let current_date = Local::now().format("%Y/%m/%d").to_string();
    let commit_message = format!("commit {}", current_date);

    // Commit the changes with the generated commit message
    if let Err(error) = run_git_command("commit", &["-m", &commit_message]) {
        eprintln!("{}", error);
        exit(1);
    }
    println!("Created a commit with message: \"{}\"", commit_message);

    // Push changes to the remote repository (main branch)
    if let Err(error) = run_git_command("push", &["origin", "main"]) {
        eprintln!("{}", error);
        exit(1);
    }
    println!("Successfully pushed changes to remote repository.");
}
