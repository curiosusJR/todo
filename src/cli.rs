use std::fmt::Display;

use clap::{arg, crate_description, crate_name, crate_version, value_parser, Command, ValueEnum};

#[derive(ValueEnum, Clone, Copy)]
pub(crate) enum Language {
    Ja,
    En,
    Zh,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

#[derive(ValueEnum, Clone, Copy)]
pub(crate) enum Shell {
    Bash,
    Zsh,
    Fish,
}

impl Display for Shell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

#[derive(ValueEnum, Clone, Copy)]
pub(crate) enum Part {
    Doing,
    Done,
    Todo,
    All,
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

pub(crate) const ADD: &str = "add";
pub(crate) const CLEAR: &str = "clear";
pub(crate) const COMPLETION: &str = "completion";
pub(crate) const CONTINUE: &str = "continue";
pub(crate) const DELETE: &str = "delete";
pub(crate) const DONE: &str = "done";
pub(crate) const EDIT: &str = "edit";
pub(crate) const LIST: &str = "list";
pub(crate) const RECORD: &str = "begin";
pub(crate) const REPORT: &str = "report";
pub(crate) const SHOW: &str = "show";
pub(crate) const SORT: &str = "sort";
pub(crate) const SWAP: &str = "swap";
pub(crate) const UNCONTINUE: &str = "uncontinue";
pub(crate) const UNDONE: &str = "undone";
pub(crate) const UNRECORD: &str = "unrecord";

pub(crate) fn build() -> Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new(LIST).about("show todo list").arg(
                arg!(<PART>)
                    // .long("part")
                    // .short('p')
                    .value_parser(value_parser!(Part))
                    .required(false),
            ),
        )
        .subcommand(Command::new(CLEAR).about("clear todo list"))
        .subcommand(
            Command::new(ADD)
                .about("add the task")
                .arg(arg!(<TASK>).required(true)),
        )
        .subcommand(
            Command::new(DELETE)
                .about("delete the task")
                .arg(arg!(<INDEX>).required(true)),
        )
        .subcommand(
            Command::new(EDIT)
                .about("edit the task description")
                .arg(arg!(<INDEX>).required(true))
                .arg(arg!(<TASK>).required(true)),
        )
        .subcommand(
            Command::new(DONE)
                .about("done the task and record elapsed time")
                .arg(arg!(<INDEX>).required(true))
                .arg(arg!(<TIME>).required(false)),
        )
        .subcommand(
            Command::new(UNDONE)
                .about("undone the task")
                .arg(arg!(<INDEX>).required(true)),
        )
        .subcommand(
            Command::new(RECORD)
                .about("record the beginning time point")
                .arg(arg!(<INDEX>).required(true))
                .arg(arg!(<TIME>).required(false)),
        )
        .subcommand(
            Command::new(UNRECORD)
                .about("unrecord elapsed time (same to undone)")
                .arg(arg!(<INDEX>).required(true)),
        )
        .subcommand(
            Command::new(SHOW)
                .about("show the task")
                .arg(arg!(<INDEX>).required(true)),
        )
        .subcommand(Command::new(SORT).about("sort tasks"))
        .subcommand(
            Command::new(SWAP)
                .about("swap two tasks")
                .arg(arg!(<INDEX1>).required(true))
                .arg(arg!(<INDEX2>).required(true)),
        )
        .subcommand(
            Command::new(REPORT)
                .about("report today's achievements")
                .arg(arg!(<COMMENT>).required(false))
                .arg(arg!(<TITLE>).required(false))
                .arg(
                    arg!(<LANG>)
                        .long("lang")
                        .short('l')
                        .value_parser(value_parser!(Language))
                        .required(false),
                ),
        )
        .subcommand(Command::new(CONTINUE).about("continue todo list"))
        .subcommand(Command::new(UNCONTINUE).about("uncontinue todo list"))
        .subcommand(
            Command::new(COMPLETION)
                .about("print shell completion")
                .arg(
                    arg!(<SHELL>)
                        .long("shell")
                        .short('s')
                        .value_parser(value_parser!(Shell))
                        .required(false),
                ),
        )
}
