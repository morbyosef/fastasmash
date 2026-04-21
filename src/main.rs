use std::path::PathBuf;
use clap::{Parser, Subcommand};

use crate::{commands::count::count, parser::fasta::parse_file};

mod parser;
mod model;
mod commands;


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    fasta_file: PathBuf,

    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Count {

    }
}

fn main() {
    let cli = Cli::parse();
    let fasta_file = parse_file(cli.fasta_file.clone());
    

    match &cli.command {
        Some(Commands::Count {}) => {
            let file_path = cli.fasta_file.display().to_string();
            let records_count = count(fasta_file);
            println!("File {file_path} has {records_count} records")
        },
        None => todo!(),
    }
}
