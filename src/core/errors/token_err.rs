use std::fmt::Display;

pub enum TokenErr {
    UnexpectedSequence(String)
}

impl Display for TokenErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", match self {
            TokenErr::UnexpectedSequence(sequence) => format!("Could not parse the sequence: {}", sequence)
        })
    }
}