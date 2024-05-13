# Scatternotes Cli

A CLI application to create an manage unordered notes.

Best used together with the neovim extension.

## Usage

Create notes in a directory (default `~/notes`):

```md
This is a #test #note.

You can #create #tags like this:

    #this-is-a-tag

Tags are used for searching notes.
```

All notes are stored in the same directory.
The names of the files are random.

All commands shown below also have shorthands.\
To view them run:

    scatternotes --help
    scatternotes <command> --help

You can generate new names with:

    scatternotes generate
    scatternotes generate 20

You can list all notes with:

    scatternotes list
    scatternotes list --with-tags

You can search for notes with:

    scatternotes search <tag1> <tag2> <...> <tagn>
    scatternotes search <tag1> <tag2> <...> <tagn> --with-tags

You can commit the notes with:

    # message: "update notes"
    scatternotes commit

You can clean the notes with:

    # delete all notes containing #just-a-test tag
    # rename all non confirming notes
    scatternotes clean

## Installation

You can use cargo to install the cli application.

    cargo install --locked scatternotes

