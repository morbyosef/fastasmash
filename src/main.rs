use std::path::PathBuf;
use clap::{Parser, Subcommand};

use crate::{commands::count::count, commands::header::header, commands::view::view, parser::fasta::parse_file};

mod parser;
mod model;
mod commands;


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    fasta_file: PathBuf,

    #[arg(long)]
    id: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Count {},
    Header {},
    View {},
}

fn main() {
    let cli = Cli::parse();
    
    let fasta_file = parse_file(cli.fasta_file, cli.id.as_deref());

    match &cli.command {
        Some(Commands::Count {}) => {
            let file_path = fasta_file.path.display().to_string();
            let records_count = count(&fasta_file);
            println!("File {file_path} has {records_count} records")
        }
        Some(Commands::Header {}) => {
            header(&fasta_file);
        }
        Some(Commands::View {}) => {
            view(&fasta_file);
        }
        None => {}
    }
}
