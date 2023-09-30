pub struct WhiteSpace;

impl WhiteSpace {
    pub fn parse(c: &char) -> bool {
        c == &' '
    }
}