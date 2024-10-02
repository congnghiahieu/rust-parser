use crate::cli::Args;
use cargo_toml::Manifest;
use std::fs;
use std::io::{self};
use std::path::Path;
use syn;

pub fn parse_by_syn(args: &Args) {
    let rust_callback = |abs_filepath: &str| {
        if args.write_stdout {
            println!("{:#?}", abs_filepath);
        }

        let parse_result = parse_ast(abs_filepath);
        if parse_result.is_err() {
            if args.write_stderr {
                eprintln!("{:#?}", parse_result.unwrap_err());
            }
            return;
        }

        let ast = parse_result.unwrap();

        if args.write_text {
            let out_text = format!("{:#?}", ast);
            write_output_format(
                &out_text,
                abs_filepath,
                &args.abs_input_dir,
                &args.abs_output_dir,
                Format::Text,
            );
        }

        if args.write_json {
            let out_json = syn_serde::json::to_string_pretty(&ast);
            write_output_format(
                &out_json,
                abs_filepath,
                &args.abs_input_dir,
                &args.abs_output_dir,
                Format::Json,
            );
        }
    };

    let toml_callback = |abs_filepath: &str| {
        if args.write_stdout {
            println!("{:#?}", abs_filepath);
        }

        let parse_result = parse_cargo_toml(abs_filepath);
        if parse_result.is_err() {
            if args.write_stderr {
                eprintln!("{:#?}", parse_result.unwrap_err());
            }
            return;
        }

        let manifest = parse_result.unwrap();

        if args.write_cargo_toml {
            let out_json = serde_json::to_string_pretty(&manifest).unwrap();
            write_output_format(
                &out_json,
                abs_filepath,
                &args.abs_input_dir,
                &args.abs_output_dir,
                Format::Json,
            );
        }
    };

    traverse_folder(
        &args.abs_input_dir,
        &rust_callback,
        &toml_callback,
        args.src_only,
    );
}

fn traverse_folder(
    folder_path: &str,
    rust_callback: &impl Fn(&str) -> (),
    toml_callback: &impl Fn(&str) -> (),
    parse_src_only: bool,
) {
    let dir = fs::read_dir(folder_path).unwrap();
    let entries = dir
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    entries.iter().for_each(|entry| {
        if entry.is_dir() {
            if !parse_src_only {
                traverse_folder(
                    entry.to_str().unwrap(),
                    rust_callback,
                    toml_callback,
                    parse_src_only,
                );
                return;
            }

            let folder_name = entry.file_name().unwrap().to_str().unwrap();
            if !(folder_name == "src") {
                return;
            }

            traverse_folder(
                entry.to_str().unwrap(),
                rust_callback,
                toml_callback,
                parse_src_only,
            );
            return;
        }

        if entry.ends_with("Cargo.toml") {
            toml_callback(entry.to_str().unwrap());
            return;
        }

        if let Some(file_extension) = entry.extension() {
            if file_extension != "rs" {
                return;
            }
            rust_callback(entry.to_str().unwrap());
        };
    });
}

fn parse_ast(filepath: &str) -> Result<syn::File, String> {
    let Ok(code) = fs::read_to_string(filepath) else {
        return Err(format!("Error reading file: {:#?}", filepath));
    };

    let Ok(ast) = syn::parse_file(&code) else {
        return Err(format!("Error parsing file: {:#?}", filepath));
    };

    Ok(ast)
}

pub fn parse_cargo_toml(cargo_toml_path: &str) -> Result<Manifest, String> {
    let Ok(mut manifest) = Manifest::from_path(cargo_toml_path) else {
        return Err(format!(
            "Error reading Cargo.toml at {:#?}",
            cargo_toml_path
        ));
    };

    manifest
        .complete_from_path(Path::new(cargo_toml_path))
        .unwrap();
    Ok(manifest)
}

enum Format {
    Text,
    Json,
}

fn write_output_format(
    content: &str,
    abs_filepath: &str,
    abs_input_dir: &str,
    abs_output_dir: &str,
    format: Format,
) {
    let out_path = String::from(abs_filepath).replace(abs_input_dir, abs_output_dir);

    match format {
        Format::Text => {
            let out_path_text = Path::new(&out_path).with_extension("txt");
            write_output(content, out_path_text.to_str().unwrap());
        }
        Format::Json => {
            let out_path_json = Path::new(&out_path).with_extension("json");
            write_output(content, out_path_json.to_str().unwrap());
        }
    }
}

fn write_output(content: &str, out_path: &str) {
    fs::create_dir_all(Path::new(out_path).parent().unwrap()).unwrap();
    fs::write(out_path, content).unwrap();
}
