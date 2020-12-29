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

    #[test]
    fn test_add_crossword_better() {
        let words0 = vec![
            "two",
            "words",
        ];
        let words1 = vec![
            "different",
            "words",
        ];

        let crossword0 = initialise_crossword(&words0);
        let crossword1 = initialise_crossword(&words1);

        let mut best_crosswords = vec![crossword1];

        assert_eq!(1, best_crosswords.len());
        assert!(best_crosswords[0] != crossword0);

        add_crossword(Comparison::Better, &crossword0, &mut best_crosswords);
        assert_eq!(1, best_crosswords.len());
        assert_eq!(crossword0, best_crosswords[0]);
    }

    #[test]
    fn test_add_crossword_as_good() {
        let words0 = vec![
            "two",
            "words",
        ];
        let words1 = vec![
            "different",
            "words",
        ];

        let crossword0 = initialise_crossword(&words0);
        let crossword1 = initialise_crossword(&words1);

        let mut best_crosswords = vec![crossword1];

        assert_eq!(1, best_crosswords.len());
        assert!(!best_crosswords.contains(&crossword0));

        add_crossword(Comparison::AsGood, &crossword0, &mut best_crosswords);
        assert_eq!(2, best_crosswords.len());
        assert!(best_crosswords.contains(&crossword0));
    }
}

