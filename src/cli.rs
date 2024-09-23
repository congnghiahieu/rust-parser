use clap::{arg, command, value_parser, ArgAction, Command};

pub struct Args {
    pub input: String,
    pub output: String,
}

fn get_command() -> Command {
    command!()
        .about("Personal Rust Parser")
        .arg(arg!(--input <VALUE>).default_value("examples"))
        .arg(arg!(--output <VALUE>).default_value("out"))
}

pub fn get_args() -> Args {
    let matches = get_command().get_matches();

    let input_dir = matches
        .get_one::<String>("input")
        .expect("Failed to get input directory");
    let output_dir = matches
        .get_one::<String>("output")
        .expect("Failed to get output directory");

    Args {
        input: input_dir.to_string(),
        output: output_dir.to_string(),
    }
}
