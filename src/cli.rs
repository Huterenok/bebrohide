use clap::{value_parser, Arg, ArgAction, Command};

//Retrieve all possible commands with their args
pub fn build_cli() -> Command {
    let hide_command = Command::new("hide")
        .about("Hides your file to another file")
        .arg(
            Arg::new("SOURCE_FILE")
                .long("source-file")
                .short('s')
                .help("Path to file where you want to hide your file")
                .required(true)
                .num_args(1)
                .value_parser(value_parser!(String)),
        )
        .arg(
            Arg::new("HIDDEN_FILE")
                .long("hidden-file")
                .short('r')
                .help("Path to file which you want to hide")
                .required(true)
                .num_args(1)
                .value_parser(value_parser!(String)),
        )
        .arg(
            Arg::new("HIDE_INSIDE")
                .long("hide-inside")
                .short('i')
                .help("(Optional) Path to place where will be generated output file. (Default - source file)")
                .num_args(1)
                .value_parser(value_parser!(String)),
        );

    let reveal_command = Command::new("reveal")
        .about("Reveals your hidden file")
        .arg(
            Arg::new("HIDDEN_FILE")
                .long("hidden-file")
                .short('r')
                .help("Path to file which you want to decode")
                .required(true)
                .num_args(1)
                .value_parser(value_parser!(String)),
        )
        .arg(
            Arg::new("DESTINATION_FOLDER")
                .long("destination-folder")
                .short('d')
                .help("Path to folder where decoded file will be generated")
                .required(true)
                .num_args(1)
                .value_parser(value_parser!(String)),
        )
        .arg(
            Arg::new("CLEAR")
                .long("clear")
                .short('c')
                .help("(Optional) Clears file from hidden part. (Default - false)")
                .action(ArgAction::SetTrue),
        );

    let check_command = Command::new("check")
        .about("Checks if file has decoded hidden file inside it")
        .arg(
            Arg::new("HIDDEN_FILE")
                .long("hidden-file")
                .short('r')
                .help("Path to file which you want to check")
                .required(true)
                .num_args(1)
                .value_parser(value_parser!(String)),
        );

    Command::new("bebrohide")
        .subcommand(hide_command)
        .subcommand(reveal_command)
        .subcommand(check_command)
        .arg_required_else_help(true)
}
