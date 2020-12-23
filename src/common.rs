
#[derive(Debug)]
pub struct WordUsage<'a> {
    pub word: &'a str,
    pub currently_in_use: bool,
}

impl PartialEq for WordUsage<'_> {
    fn eq(&self, other: &Self) -> bool {
        return self.word.eq(other.word);
    }
}

