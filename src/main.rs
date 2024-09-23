mod ast;
mod cli;
mod utils;

use std::fs;
use std::io::{self, Read};
use std::path;
use std::str::FromStr;
use syn;
use syn_serde::json;

fn main() -> io::Result<()> {
    let in_dir = "examples";
    let out_dir = "out";

    traverse_folder(in_dir)?;

    Ok(())
}

fn parse_file(infile_path: &str) {
    let mut infile = fs::File::open(infile_path).unwrap();
    let mut infile_code = String::new();
    infile.read_to_string(&mut infile_code);

    let ast = syn::parse_file(&infile_code).unwrap();

    println!("{:#?}", infile_path);

    write_output_rawstr(&ast, infile_path);
    write_output_jsonstr(&ast, infile_path);
}

fn write_output_rawstr(ast: &syn::File, infile_path: &str) {
    let infile_path = path::Path::new(infile_path);
    let outfolder_path = String::from_str(infile_path.parent().unwrap().to_str().unwrap())
        .unwrap()
        .replace("examples", "out/raw");
    let outfile_filename = format!("{}.txt", infile_path.file_stem().unwrap().to_str().unwrap());
    let outfile_filepath = path::Path::new(&outfolder_path).join(outfile_filename);
    let out_rawstr = format!("{:#?}", ast);

    fs::create_dir_all(outfolder_path);
    fs::write(outfile_filepath, out_rawstr).unwrap();
}

fn write_output_jsonstr(ast: &syn::File, infile_path: &str) {
    let infile_path = path::Path::new(infile_path);
    let outfolder_path = String::from_str(infile_path.parent().unwrap().to_str().unwrap())
        .unwrap()
        .replace("examples", "out/json");
    let outfile_filename = format!(
        "{}.json",
        infile_path.file_stem().unwrap().to_str().unwrap()
    );
    let outfile_filepath = path::Path::new(&outfolder_path).join(outfile_filename);
    let out_jsonstr = json::to_string_pretty(ast);

    fs::create_dir_all(outfolder_path);
    fs::write(outfile_filepath, out_jsonstr).unwrap();
}

fn traverse_folder(folder_path: &str) -> io::Result<()> {
    let entries = fs::read_dir(folder_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.iter().for_each(|entry| {
        if entry.is_dir() {
            traverse_folder(entry.to_str().unwrap());
        } else {
            parse_file(entry.to_str().unwrap());
        }
    });

    Ok(())
}
