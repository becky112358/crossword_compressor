#[cfg(test)]
mod tests {
    use crate::crossword::{initialise_crossword};
    use crate::options::*;

    #[test]
    fn test_get_new_start_end_row() {
        let word_and_letter = WordAndLetter {
            word_index: 22,
            word: "blue",
            letter: 'u',
            letter_index: 2,
            n_letters_after: 1,
        };

        assert_eq!((-4, -1, -2), get_new_start_end_row(&[-2, -2], Direction::Across, &word_and_letter));
        assert_eq!((-2, 1, 0), get_new_start_end_row(&[0, 0], Direction::Across, &word_and_letter));
        assert_eq!((6, 9, 3), get_new_start_end_row(&[8, 3], Direction::Across, &word_and_letter));
        assert_eq!((-4, -1, -2), get_new_start_end_row(&[-2, -2], Direction::Down, &word_and_letter));
    }

    #[test]
    fn test_get_old_start_end_row() {
        let mut data = CrossData {
            position: [-3, 8],
            direction: Direction::Across,
            order: 6,
        };

        assert_eq!((-3, 1, 8), get_old_start_end_row("skies", &data));

        data.direction = Direction::Down;
        assert_eq!((8, 12, -3), get_old_start_end_row("skies", &data));
    }

    #[test]
    fn test_check_connecting_word() {
        let words = vec!["lonely".to_string()];
        let crossword = initialise_crossword(&words);

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
    fn test_get_start_position() {
        let word_and_letter = WordAndLetter {
            word_index: 3,
            word: "hello",
            letter: 'l',
            letter_index: 3,
            n_letters_after: 1,
        };

        assert_eq!([4, 6], get_start_position(&[7, 6], Direction::Across, &word_and_letter));
        assert_eq!([-3, 8], get_start_position(&[-3, 11], Direction::Down, &word_and_letter));
    }

    #[test]
    fn test_is_duplicate() {
        let words = vec![
            "three".to_string(),
            "words".to_string(),
            "here".to_string(),
            "surprise".to_string(),
        ];

        let mut crossword0 = initialise_crossword(&words);
        let crossword1 = initialise_crossword(&words);
        let mut best_crosswords = vec![crossword1];

        crossword0.words[1].cross = Some(CrossData{ position: [2, -2], direction: Direction::Down, order: 1 });
        crossword0.words[2].cross = Some(CrossData{ position: [4, -3], direction: Direction::Down, order: 2 });

        best_crosswords[0].words[1].cross = Some(CrossData{ position: [2, -2], direction: Direction::Down, order: 2 });
        best_crosswords[0].words[2].cross = Some(CrossData{ position: [4, -3], direction: Direction::Down, order: 1 });
        best_crosswords[0].words[3].cross = Some(CrossData{ position: [2, 2], direction: Direction::Across, order: 3 });

        assert!(is_duplicate(&crossword0, &best_crosswords));

        best_crosswords[0].words[1].cross = Some(CrossData{ position: [2, -2], direction: Direction::Down, order: 3 });
        best_crosswords[0].words[3].cross = Some(CrossData{ position: [2, 2], direction: Direction::Across, order: 2 });

        assert!(!is_duplicate(&crossword0, &best_crosswords));

        best_crosswords[0].words[1].cross = Some(CrossData{ position: [2, -2], direction: Direction::Down, order: 2 });
        best_crosswords[0].words[2].cross = Some(CrossData{ position: [4, -1], direction: Direction::Down, order: 1 });
        best_crosswords[0].words[3].cross = None;

        assert!(!is_duplicate(&crossword0, &best_crosswords));
    }

    #[test]
    fn test_add_crossword_first() {
        let words = vec![
            "two".to_string(),
            "words".to_string(),
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
            "two".to_string(),
            "words".to_string(),
        ];
        let words1 = vec![
            "different".to_string(),
            "words".to_string(),
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
    fn test_add_crossword_as_good_worse_seed_duplicate() {
        let words0 = vec![
            "two".to_string(),
            "words".to_string(),
        ];
        let words1 = vec![
            "different".to_string(),
            "words".to_string(),
        ];

        let crossword0 = initialise_crossword(&words0);
        let crossword1 = initialise_crossword(&words1);

        let mut best_crosswords = vec![crossword1];

        assert_eq!(1, best_crosswords.len());
        assert!(!best_crosswords.contains(&crossword0));

        add_crossword(Comparison::Worse, &crossword0, &mut best_crosswords);
        assert_eq!(1, best_crosswords.len());
        assert!(!best_crosswords.contains(&crossword0));

        add_crossword(Comparison::SeedDuplicate, &crossword0, &mut best_crosswords);
        assert_eq!(1, best_crosswords.len());
        assert!(!best_crosswords.contains(&crossword0));

        add_crossword(Comparison::AsGood, &crossword0, &mut best_crosswords);
        assert_eq!(2, best_crosswords.len());
        assert!(best_crosswords.contains(&crossword0));
    }
}

