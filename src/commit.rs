use std::process::Command;

use crate::config::Config;

pub fn commit_notes(config: &Config) {
    let git_status = Command::new("git")
        .args(["status", "--short"])
        .current_dir(config.path())
        .output();
    match git_status {
        Ok(result) => {
            if !result.status.success() {
                eprintln!(
                    "ERROR 'git status' exited with a error status: {}",
                    result.status
                );
                if let Ok(stderr) = String::from_utf8(result.stderr) {
                    eprintln!("{}", stderr);
                }
                return;
            }
            let Ok(stdout) = String::from_utf8(result.stdout) else {
                return;
            };
            let not_changes = stdout.is_empty();
            if not_changes {
                println!("INFO no changes detected");
                return;
            }
            let num_notes = stdout.chars().filter(|char| matches!(char, '\n')).count();
            println!("INFO committing {} notes", num_notes);
        }
        Err(error) => {
            eprintln!("ERROR could not execute 'git status'");
            eprintln!("{}", error);
            return;
        }
    }

    let git_add = Command::new("git")
        .args(["add", "--all"])
        .current_dir(config.path())
        .status();
    match git_add {
        Ok(result) => {
            if !result.success() {
                eprintln!("ERROR 'git add' exited with a error status: {}", result);
                return;
            }
        }
        Err(error) => {
            eprintln!("ERROR could not execute 'git add'");
            eprintln!("{}", error);
            return;
        }
    };

    let git_commit = Command::new("git")
        .args(["commit", "-m", "\"update notes\""])
        .current_dir(config.path())
        .status();
    match git_commit {
        Ok(result) => {
            if !result.success() {
                eprintln!("ERROR 'git commit' exited with a error status: {}", result);
                return;
            }
        }
        Err(error) => {
            eprintln!("ERROR could not execute 'git commit'");
            eprintln!("{}", error);
            return;
        }
    }

    let git_pull = Command::new("git")
        .arg("pull")
        .current_dir(config.path())
        .status();
    match git_pull {
        Ok(result) => {
            if !result.success() {
                eprintln!("ERROR 'git pull' exited with a error status: {}", result);
                return;
            }
        }
        Err(error) => {
            eprintln!("ERROR could not execute 'git pull'");
            eprintln!("{}", error);
            return;
        }
    }

    let git_push = Command::new("git")
        .arg("push")
        .current_dir(config.path())
        .status();
    match git_push {
        Ok(result) => {
            if !result.success() {
                eprintln!("ERROR 'git push' exited with a error status: {}", result);
                return;
            }
        }
        Err(error) => {
            eprintln!("ERROR could not execute 'git push'");
            eprintln!("{}", error);
            return;
        }
    }

    println!("INFO commit was successful");

    ()
}
