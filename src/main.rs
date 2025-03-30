use clap::Parser;
use std::{fs, process::ExitCode};

use asprite_json::AspriteJson;
use sparrow_xml::TextureAtlas;

mod asprite_json;
mod sparrow_xml;
mod util;

#[derive(Parser, Debug)]
#[command(about, version)]
struct Cli {
    #[arg(short = 'i')]
    in_file: String,

    #[arg(short = 'o')]
    out_file: String,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let input_res = fs::read_to_string(cli.in_file);
    if let Err(e) = input_res {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }
    let input = input_res.unwrap();
    let json_res = serde_json::from_str::<AspriteJson>(&input);
    if let Err(e) = json_res {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }
    let json = json_res.unwrap();
    let xml: TextureAtlas = json.into();
    if let Err(e) = fs::write(cli.out_file, format!("{}", xml)) {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
