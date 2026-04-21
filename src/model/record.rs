
pub struct FastaRecord {
    pub id: String,
    pub description: String,
    pub sequence: String,
}

impl From<String> for FastaRecord {
    fn from(record_string: String) -> Self {
        let mut lines = record_string.lines();
        let header = lines.next().expect("Invalid Record");
        let (id, description) = header.split_once(' ').unwrap_or((header, ""));
        let sequence = lines.map(str::trim).collect();
        FastaRecord {
            id: id.to_string(),
            description: description.to_string(),
            sequence,
        }
    }
}
