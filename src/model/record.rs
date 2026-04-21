
pub struct FastaRecord {
    pub id: String,
    pub description: String,
    pub sequence: String,
}

impl FastaRecord {
    pub fn from(string_record: String) -> FastaRecord {
        let mut lines = string_record.lines();
        let header = lines.next().unwrap_or_default();
        let (id, description) = header.split_once(' ').unwrap_or((header, ""));
        let sequence = lines.collect::<String>();
        FastaRecord {
            id: id.to_string(),
            description: description.to_string(),
            sequence,
        }
    }
}
