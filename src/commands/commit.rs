use std::process::{Command, Stdio};

use termfmt::{CommandOutputError, CommandStatusError};

use crate::config::Config;
use crate::output::{OutputFmt, Term};

pub const NAME: &'static str = "commit";

pub fn command() -> clap::Command {
    clap::Command::new("commit").about("commit the changes using git and push them to the remote")
}
pub fn run(term: &mut Term, config: &Config) {
    term.headline("RUNNING");
    term.command("git status --short");
    let Ok(stdout) = Command::new("git")
        .args(["status", "--short"])
        .current_dir(config.path())
        .output()
        .output_error()
        .map_err(|error| term.error(error))
        .map(|output| String::from_utf8(output.stdout).unwrap_or(String::new()))
    else {
        return;
    };

    let not_changes = stdout.is_empty();
    if not_changes {
        term.info("no changes were detected!");
        return;
    }

    let num_notes = stdout.chars().filter(|char| matches!(char, '\n')).count();
    if num_notes == 1 {
        term.info("committing one note!");
    } else {
        term.info(format!("committing {} notes!", num_notes));
    }

    term.headline("RUNNING");
    term.command("git add --all");
    if Command::new("git")
        .args(["add", "--all"])
        .current_dir(config.path())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .status_error()
        .map_err(|error| term.error(error))
        .is_err()
    {
        return;
    }

    term.command("git commit -m \"update notes\"");
    if Command::new("git")
        .args(["commit", "-m", "\"update notes\""])
        .current_dir(config.path())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .status_error()
        .map_err(|error| term.error(error))
        .is_err()
    {
        return;
    }

    term.command("git pull --rebase");
    if Command::new("git")
        .args(["pull", "--rebase"])
        .current_dir(config.path())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .status_error()
        .map_err(|error| term.error(error))
        .is_err()
    {
        return;
    }

    term.command("git push");
    if Command::new("git")
        .arg("push")
        .current_dir(config.path())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .status_error()
        .map_err(|error| term.error(error))
        .is_err()
    {
        return;
    }

    term.info("the commit was successful!");
}
