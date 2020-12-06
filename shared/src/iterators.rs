use std::str::Split;

pub trait BlockSplit {
    fn blocks(&self) -> Split<&str>;
}

impl BlockSplit for &str {
    fn blocks(&self) -> Split<&str> {
        self.split("\n\n")
    }
}
