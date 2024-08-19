use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::code::Code;
use crate::config::Config;
use crate::note::Note;
use crate::output::{OutputFmt, Term};

pub const NAME: &str = "code";

pub const ARG_LANG: &str = "lang";
pub const ARG_NOTE: &str = "note";
pub const ARG_RUN: &str = "run";

pub fn command() -> Command {
    Command::new(NAME)
        .args([
            Arg::new(ARG_LANG)
                .long(ARG_LANG)
                .help("the language to find in the note"),
            Arg::new(ARG_NOTE)
                .num_args(1..)
                .required(true)
                .help("the notes to run the code from"),
            Arg::new(ARG_RUN)
                .long(ARG_RUN)
                .action(ArgAction::SetTrue)
                .help("run the code from the note"),
        ])
        .about("run code in notes")
}

pub fn run(command: &ArgMatches, term: &mut Term, config: &Config) {
    let note_name: &String = command.get_one(ARG_NOTE).unwrap();

    let note_path = config.note(note_name);
    let Ok(note) = Note::load(note_path.clone()) else {
        term.error(format_args!("cannot open file {}", note_path.display()));
        return;
    };

    let languages: Option<&String> = command.get_one(ARG_LANG);
    let code = match languages {
        Some(language) => Code::search_language(&note, language),
        None => Code::search(&note),
    };

    let Some(code) = code else {
        term.error(format_args!(
            "no code found in note {}",
            note_path.display()
        ));
        return;
    };

    code.save(config);
    term.info(format_args!(
        "generated code file {}",
        code.path(config).display()
    ));

    if command.get_flag(ARG_RUN) {
        let output = code.run(config).unwrap();
        term.headline("OUTPUT");
        term.command_output(&output);
    }
}
