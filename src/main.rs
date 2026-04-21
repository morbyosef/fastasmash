use std::path::PathBuf;
use clap::{Parser, Subcommand};

use crate::{commands::count::count, commands::header::header, commands::view::view, parser::fasta::{filter_by_id, parse_file}};

mod parser;
mod model;
mod commands;


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    fasta_file: PathBuf,

    #[arg(long, help = "Filter records by ID (substring match: --id seq matches seq1, seq2, etc.)")]
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

    let mut fasta_file = parse_file(cli.fasta_file);
    if let Some(id) = cli.id.as_deref() {
        fasta_file = filter_by_id(fasta_file, id);
    }

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
