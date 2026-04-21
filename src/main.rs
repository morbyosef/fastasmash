use std::path::PathBuf;
use clap::{Parser, Subcommand};

use crate::{commands::count::count, commands::header::header, parser::fasta::parse_file};

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
    Count {},
    Header {},
}

fn main() {
    let cli = Cli::parse();
    
    match &cli.command {
        Some(Commands::Count {}) => {
            let fasta_file = parse_file(cli.fasta_file);
            let file_path = fasta_file.path.display().to_string();
            let records_count = count(&fasta_file);
            println!("File {file_path} has {records_count} records")
        }
        Some(Commands::Header {}) => {
            let fasta_file = parse_file(cli.fasta_file);
            header(&fasta_file);
        }
        None => {}
    }
}
