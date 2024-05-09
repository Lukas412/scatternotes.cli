use std::process::{Command, Stdio};

use termfmt::{
    command::{CmdOutErr, CmdStatErr},
    strategies::TermFmtStrategy,
    TermError, TermStyle,
};

use crate::config::Config;

pub fn commit_notes<'a>(config: &Config, fmt: &impl TermFmtStrategy) {
    fmt.headline("RUNNING".fg_green().bold());
    fmt.action("git status --short");
    let Some(stdout) = Command::new("git")
        .args(["status", "--short"])
        .current_dir(config.path())
        .output()
        .cmd_out_err()
        .termerr()
        .and_then(|output| String::from_utf8(output.stdout).ok())
    else {
        return;
    };

    let not_changes = stdout.is_empty();
    if not_changes {
        fmt.info("no changes were detected!");
        return;
    }
    let num_notes = stdout.chars().filter(|char| matches!(char, '\n')).count();

    if num_notes == 1 {
        fmt.info("committing one note!");
    } else {
        fmt.info(format_args!("committing {} notes!", num_notes));
    }

    fmt.headline("RUNNING".fg_green().bold());
    fmt.action("git add --all");
    if Command::new("git")
        .args(["add", "--all"])
        .current_dir(config.path())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .cmd_stat_err()
        .termerr()
        .is_none()
    {
        return;
    }

    fmt.action("git commit -m \"update notes\"");
    if Command::new("git")
        .args(["commit", "-m", "\"update notes\""])
        .current_dir(config.path())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .cmd_stat_err()
        .termerr()
        .is_none()
    {
        return;
    }

    fmt.action("git pull");
    if Command::new("git")
        .arg("pull")
        .current_dir(config.path())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .cmd_stat_err()
        .termerr()
        .is_none()
    {
        return;
    }

    fmt.action("git push");
    if Command::new("git")
        .arg("push")
        .current_dir(config.path())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .cmd_stat_err()
        .termerr()
        .is_none()
    {
        return;
    }

    fmt.info("the commit was successful!");
}
