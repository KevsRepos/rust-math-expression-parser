use std::fmt::Display;

#[derive(Debug)]
pub enum ScanErr {
    UnexpectedSequence(String)
}

impl Display for ScanErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", match self {
            ScanErr::UnexpectedSequence(sequence) => format!("Could not parse the sequence: `{}`", sequence)
        })
    }
}