use clap::{arg, command, ArgAction, Command};
use std::env;
use std::path::Path;

pub struct Args {
    pub abs_input_dir: String,
    pub abs_output_dir: String,
    pub src_only: bool,
    pub write_stdout: bool,
    pub write_stderr: bool,
    pub write_text: bool,
    pub write_json: bool,
    pub write_cargo_toml: bool,
}

fn get_command() -> Command {
    command!()
        .next_line_help(true)
        .arg(
            arg!(
                -i --input <INPUT_FOLDER> "Rust project input folder"
            )
            .required(true),
        )
        .arg(
            arg!(
                -o --output <OUTPUT_FOLDER> "Parsed output folder"
            )
            .required(true),
        )
        .arg(
            arg!(
               --"src-only" <OUTPUT_FOLDER> "Only parse src folder"
            )
            .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(
                --stdout <OUTPUT_FOLDER> "Write output to stdout"
            )
            .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(
                --stderr <OUTPUT_FOLDER> "Write error to stderr"
            )
            .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(
                --text <OUTPUT_FOLDER> "Write output as text"
            )
            .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(
                --json <OUTPUT_FOLDER> "Write output as text"
            )
            .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(
                --"cargo-toml" <OUTPUT_FOLDER> "Write cargo.toml to output"
            )
            .action(ArgAction::SetTrue),
        )
}

fn get_valid_folder(s: &str, not_exist_ok: bool) -> Result<String, String> {
    let Ok(cwd) = env::current_dir() else {
        return Err("Failed to get current working directory".to_owned());
    };

    let full_path = cwd.join(Path::new(s));

    if !full_path.exists() && !not_exist_ok {
        return Err(format!("Folder does not exist: {:#?}", full_path));
    }

    if full_path.exists() && !full_path.is_dir() {
        return Err(format!("Path is not a folder: {:#?}", full_path));
    }

    Ok(full_path.to_str().unwrap().to_owned())
}

pub fn get_args() -> Args {
    let matches = get_command().get_matches();

    let input_dir = matches
        .get_one::<String>("input")
        .expect("Failed to get input directory")
        .to_owned();
    let output_dir = matches
        .get_one::<String>("output")
        .expect("Failed to get output directory")
        .to_owned();
    let src_only = matches
        .get_one::<bool>("src-only")
        .expect("Failed to get src-only flag")
        .to_owned();
    let write_stdout = matches
        .get_one::<bool>("stdout")
        .expect("Failed to get stdout flag")
        .to_owned();
    let write_stderr = matches
        .get_one::<bool>("stderr")
        .expect("Failed to get stderr flag")
        .to_owned();
    let write_text = matches
        .get_one::<bool>("text")
        .expect("Failed to get text flag")
        .to_owned();
    let write_json = matches
        .get_one::<bool>("json")
        .expect("Failed to get json flag")
        .to_owned();
    let write_cargo_toml = matches
        .get_one::<bool>("cargo-toml")
        .expect("Failed to get cargo-toml flag")
        .to_owned();

    let Ok(abs_input_dir) = get_valid_folder(&input_dir, false) else {
        panic!(
            "Input directory does not exist or is not a folder: {:#?}",
            input_dir
        );
    };

    let Ok(abs_output_dir) = get_valid_folder(&output_dir, true) else {
        panic!("Output must be a folder: {:#?}", output_dir);
    };

    Args {
        abs_input_dir,
        abs_output_dir,
        src_only,
        write_stdout,
        write_stderr,
        write_text,
        write_json,
        write_cargo_toml,
    }
}
