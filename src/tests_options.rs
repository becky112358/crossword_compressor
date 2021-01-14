#[cfg(test)]
mod tests {
    use crate::crossword::{crossword_initialise, WordCross};
    use crate::options::*;

    #[test]
    fn test_get_end_point() {
        assert_eq!(22, get_end_point(18, "hello"));
    }

    #[test]
    fn test_check_connecting_word() {
        let words = vec!["lonely".to_string()];
        let crossword = crossword_initialise(&words);

        assert!(check_connecting_word(3, 4, 0, Direction::Across, &crossword));
        assert!(check_connecting_word(3, 2, 0, Direction::Across, &crossword));
        assert!(!check_connecting_word(3, 4, 0, Direction::Down, &crossword));
        assert!(!check_connecting_word(3, 4, 1, Direction::Across, &crossword));
        assert!(!check_connecting_word(-1, 0, 0, Direction::Across, &crossword));
    }

    #[test]
    fn test_check_different_direction() {
        assert!(check_different_direction(0, 8, 3, "different", -16, -13, 10, "rows"));
        assert!(!check_different_direction(0, 4, 3, "words", 2, 6, 5, "touch"));
        assert!(!check_different_direction(9, 16, -4, "clashing", -7, 4, 12, "intersection"));
        assert!(check_different_direction(9, 18, -4, "acceptable", -7, 4, 12, "intersection"));
        assert!(check_different_direction(9, 18, -4, "acceptable", -7, 4, 18, "intersection"));
    }

    #[test]
    fn test_get_nth_letter() {
        assert_eq!('e', get_nth_letter("environment", 0));
        assert_eq!('o', get_nth_letter("environment", 5));
        assert_eq!('e', get_nth_letter("environment", 8));
        assert_eq!('t', get_nth_letter("environment", 10));
    }

    #[test]
    fn test_is_duplicate() {
        let words = vec![
            "three".to_string(),
            "words".to_string(),
            "here".to_string(),
            "surprise".to_string(),
        ];

        let mut crossword0 = crossword_initialise(&words);
        let crossword1 = crossword_initialise(&words);
        let mut best_crosswords = vec![crossword1];

        crossword0.words[1].cross = Some(CrossData{ row: -2, start_point: 2, direction: Direction::Down, order: 1 });
        crossword0.words[2].cross = Some(CrossData{ row: -3, start_point: 4, direction: Direction::Down, order: 2 });

        best_crosswords[0].words[1].cross =
            Some(CrossData{ row: -2, start_point: 2, direction: Direction::Down, order: 2 });
        best_crosswords[0].words[2].cross =
            Some(CrossData{ row: -3, start_point: 4, direction: Direction::Down, order: 1 });
        best_crosswords[0].words[3].cross =
            Some(CrossData{ row: 2, start_point: 2, direction: Direction::Across, order: 3 });

        assert!(is_duplicate(&crossword0, &best_crosswords));

        best_crosswords[0].words[1].cross =
            Some(CrossData{ row: -2, start_point: 2, direction: Direction::Down, order: 3 });
        best_crosswords[0].words[3].cross =
            Some(CrossData{ row: 2, start_point: 2, direction: Direction::Across, order: 2 });

        assert!(!is_duplicate(&crossword0, &best_crosswords));

        best_crosswords[0].words[1].cross =
            Some(CrossData{ row: -2, start_point: 2, direction: Direction::Down, order: 2 });
        best_crosswords[0].words[2].cross =
            Some(CrossData{ row: -1, start_point: 4, direction: Direction::Down, order: 1 });
        best_crosswords[0].words[3].cross = None;

        assert!(!is_duplicate(&crossword0, &best_crosswords));
    }

    #[test]
    fn test_add_crossword() {
        let words0 = vec![
            "two".to_string(),
            "words".to_string(),
        ];
        let words1 = vec![
            "different".to_string(),
            "words".to_string(),
        ];

        let crossword0 = crossword_initialise(&words0);
        let crossword1 = crossword_initialise(&words1);

        let mut best_crosswords = vec![];

        assert_eq!(0, best_crosswords.len());
        assert!(!best_crosswords.contains(&crossword1));
        add_crossword(Comparison::First, &crossword1, &mut best_crosswords);
        assert_eq!(1, best_crosswords.len());
        assert!(best_crosswords.contains(&crossword1));

        assert_eq!(1, best_crosswords.len());
        assert!(!best_crosswords.contains(&crossword0));
        add_crossword(Comparison::Worse, &crossword0, &mut best_crosswords);
        assert_eq!(1, best_crosswords.len());
        assert!(!best_crosswords.contains(&crossword0));

        assert_eq!(1, best_crosswords.len());
        add_crossword(Comparison::SeedDuplicate, &crossword0, &mut best_crosswords);
        assert_eq!(1, best_crosswords.len());
        assert!(!best_crosswords.contains(&crossword0));

        assert_eq!(1, best_crosswords.len());
        add_crossword(Comparison::AsGood, &crossword0, &mut best_crosswords);
        assert_eq!(2, best_crosswords.len());
        assert!(best_crosswords.contains(&crossword0));

        assert_eq!(2, best_crosswords.len());
        add_crossword(Comparison::Better, &crossword1, &mut best_crosswords);
        assert_eq!(1, best_crosswords.len());
        assert!(best_crosswords.contains(&crossword1));
        assert!(!best_crosswords.contains(&crossword0));

        assert!(!best_crosswords.contains(&crossword0));
        add_crossword(Comparison::Better, &crossword0, &mut best_crosswords);
        assert_eq!(1, best_crosswords.len());
        assert!(best_crosswords.contains(&crossword0));
    }

    #[test]
    fn test_remove_word() {
        let word_index = 0;
        let word_and_letter = WordAndLetter {
            word_index,
            word: "lonesome",
            letter: 'o',
            letter_index: 5,
            n_letters_after: 2,
        };
        let mut crossword = Crossword {
            words: vec![WordCross {word: "lonesome", cross: Some(CrossData {row: 3,
                                                                            start_point: 14,
                                                                            direction: Direction::Across,
                                                                            order: 0})}],
        };

        assert!(crossword.words[word_index].cross != None);
        remove_word(&word_and_letter, &mut crossword);
        assert!(crossword.words[word_index].cross == None);
    }
}

