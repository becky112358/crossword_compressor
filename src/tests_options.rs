#[cfg(test)]
mod tests {
    use crate::crossword::{initialise_crossword};
    use crate::options::*;

    #[test]
    fn test_check_no_same_direction_overlaps() {
        let words = vec![
            "two",
            "words",
        ];

        let mut crossword = initialise_crossword(&words);
        crossword.words[1].cross = Some(CrossData{ position: [2, -1], direction: Direction::Down, order: 1 });

        assert!(check_no_same_direction_overlaps(&[0, 0], &[0, 8], &Direction::Down, &crossword));
        assert!(!check_no_same_direction_overlaps(&[1, 0], &[1, 8], &Direction::Down, &crossword));
        assert!(!check_no_same_direction_overlaps(&[2, 0], &[2, 8], &Direction::Down, &crossword));
        assert!(!check_no_same_direction_overlaps(&[3, 0], &[3, 8], &Direction::Down, &crossword));
        assert!(check_no_same_direction_overlaps(&[2, 2], &[2, 5], &Direction::Across, &crossword));
        assert!(!check_no_same_direction_overlaps(&[1, 1], &[1, 6], &Direction::Across, &crossword));
    }

    #[test]
    fn test_check_row_clear() {
        let words = vec![
            "two",
            "words",
        ];

        let mut crossword = initialise_crossword(&words);
        crossword.words[1].cross = Some(CrossData{ position: [2, -1], direction: Direction::Down, order: 1 });

        assert!(check_row_clear(&Direction::Across, 1, -1, 5, &crossword));
        assert!(check_row_clear(&Direction::Across, 0, -4, -1, &crossword));
        assert!(!check_row_clear(&Direction::Across, 0, -4, 0, &crossword));
        assert!(!check_row_clear(&Direction::Across, 0, 1, 8, &crossword));
        assert!(!check_row_clear(&Direction::Across, 0, -2, 12, &crossword));
        assert!(check_row_clear(&Direction::Down, 3, -1, 3, &crossword));
        assert!(!check_row_clear(&Direction::Down, 2, 1, 2, &crossword));
    }

    #[test]
    fn test_check_other_direction_overlaps() {
        let words = vec![
            "two",
            "words",
            "third",
        ];

        let mut crossword = initialise_crossword(&words);
        crossword.words[1].cross = Some(CrossData{ position: [2, -1], direction: Direction::Down, order: 1 });

        let word_and_d = WordAndLetter {
            word_index: 2,
            word: &words[2],
            letter: 'd',
            n_letters_before: 4,
            letter_index: 4,
            n_letters_after: 0,
        };

        assert!(check_other_direction_overlaps(&[-2, 2], &Direction::Across, &word_and_d, &crossword));
        assert!(!check_other_direction_overlaps(&[-3, 2], &Direction::Across, &word_and_d, &crossword));
        assert!(!check_other_direction_overlaps(&[-2, 3], &Direction::Across, &word_and_d, &crossword));

        let word_and_t = WordAndLetter {
            word_index: 2,
            word: &words[2],
            letter: 't',
            n_letters_before: 0,
            letter_index: 0,
            n_letters_after: 4,
        };

        assert!(check_other_direction_overlaps(&[0, 0], &Direction::Down, &word_and_t, &crossword));
        assert!(!check_other_direction_overlaps(&[0, 0], &Direction::Across, &word_and_t, &crossword));
        assert!(!check_other_direction_overlaps(&[0, -1], &Direction::Down, &word_and_t, &crossword));
    }

    #[test]
    fn test_check_letter_intersections() {
        let words = vec![
            "two",
            "words",
        ];

        let mut crossword = initialise_crossword(&words);
        crossword.words[1].cross = Some(CrossData{ position: [1, 0], direction: Direction::Down, order: 1 });

        assert!(check_letter_intersections(&[2, 0], &Direction::Across, 'o', &crossword));
        assert!(!check_letter_intersections(&[2, 0], &Direction::Across, 'a', &crossword));
        assert!(check_letter_intersections(&[1, 3], &Direction::Down, 'd', &crossword));
        assert!(!check_letter_intersections(&[1, 3], &Direction::Down, 'e', &crossword));
        assert!(check_letter_intersections(&[1, 6], &Direction::Down, 'p', &crossword));
    }

    #[test]
    fn test_is_duplicate() {
        let words = vec![
            "three",
            "words",
            "here",
            "surprise",
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
    fn test_add_crossword_as_good_worse_seed_duplicate() {
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

