use crate::model::file::FastaFile;

pub fn header(fasta_file: &FastaFile) {
    for record in &fasta_file.records {
        if record.description.is_empty() {
            println!(">{}", record.id);
        } else {
            println!(">{} {}", record.id, record.description);
        }
    }
}
