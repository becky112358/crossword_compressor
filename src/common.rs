
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


pub struct Crossword {
    pub left_edge: usize,
    pub upper_edge: usize,
    pub right_edge: usize,
    pub lower_edge: usize,
    pub letters: Vec<Vec<char>>,
}

impl Crossword {
    pub fn print(&self) {
        for y in self.upper_edge..=self.lower_edge {
            for x in self.left_edge..=self.right_edge {
                print!("{}", self.letters[x][y]);
            }
            println!();
        }
    }
}

