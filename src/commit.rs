use std::process::{Command, Stdio};

use termfmt::{CommandOutputError, CommandStatusError, TermFmt};

use crate::{
    config::Config,
    output::{DataBundle, OutputData, OutputFmt},
};

pub fn commit_notes<'a>(config: &Config, term: &mut TermFmt<OutputData, DataBundle>) {
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

    term.command("git pull");
    if Command::new("git")
        .arg("pull")
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
