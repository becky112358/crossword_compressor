#[cfg(test)]
mod tests {
    use crate::crossword::initialise_crossword;
    use crate::options::*;

    #[test]
    fn test_add_crossword_first() {
        let words = vec![
            "two",
            "words",
        ];

        let crossword = initialise_crossword(&words);

        let mut best_crosswords = Vec::new();

        add_crossword(Comparison::First, &crossword, &mut best_crosswords);
        assert_eq!(1, best_crosswords.len());
        assert_eq!(crossword, best_crosswords[0]);
    }
}

