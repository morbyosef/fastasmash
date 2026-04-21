use crate::model::file::FastaFile;

pub fn view(fasta_file: &FastaFile, id: Option<&str>) {
    for record in &fasta_file.records {
        if id.is_some_and(|id| !record.id.contains(id)) {
            continue;
        }
        if record.description.is_empty() {
            println!(">{}", record.id);
        } else {
            println!(">{} {}", record.id, record.description);
        }
        println!("{}", record.sequence);
    }
}
