#[cfg(test)]
mod tests {
    use crate::crossword::*;

    #[test]
    fn test_direction_index() {
        assert_eq!(X, Direction::Across.index());
        assert_eq!(Y, Direction::Down.index());
    }

    #[test]
    fn test_direction_change() {
        assert_eq!(Direction::Down, Direction::Across.change());
        assert_eq!(Direction::Across, Direction::Down.change());
    }

    #[test]
    fn test_crossword_get_crossable_letters() {
        let crossword = helper_get_generic_crossword();

        let crossable_letters = crossword.get_crossable_letters();

        assert_eq!(vec![('a', 5, 6, Direction::Across),
                        ('l', 6, 6, Direction::Across),
                        ('p', 7, 6, Direction::Across),
                        ('h', 8, 6, Direction::Across),
                        ('a', 9, 6, Direction::Across),
                        ('b', 6, 3, Direction::Down),
                        ('r', 7, 3, Direction::Down),
                        ('a', 8, 3, Direction::Down),
                        ('v', 9, 3, Direction::Down),
                        ('o', 10, 3, Direction::Down),
                        ('d', 10, 1, Direction::Down),
                        ('e', 11, 1, Direction::Down),
                        ('l', 12, 1, Direction::Down),
                        ('t', 13, 1, Direction::Down),
                        ('a', 14, 1, Direction::Down),
                       ],
                   crossable_letters);
    }

    #[test]
    fn test_crossword_get_next_order() {
        let crossword = helper_get_generic_crossword();
        assert_eq!(3, crossword.get_next_order());
    }

    #[test]
    fn test_crossword_get_min_max() {
        let crossword = helper_get_generic_crossword();
        assert_eq!((5, 7), crossword.get_min_max());
    }

    #[test]
    fn test_crossword_all_words_crossed() {
        let mut crossword = helper_get_generic_crossword();
        assert!(!crossword.all_words_crossed());

        crossword.words[1].cross = Some(CrossData {row: 3, start_point: 10, direction: Direction::Across, order: 0});
        crossword.words[3].cross = Some(CrossData {row: 1, start_point: 6, direction: Direction::Across, order: 2});
        crossword.words[2].cross = Some(CrossData {row: 0, start_point: 2, direction: Direction::Down, order: 3});
        assert!(crossword.all_words_crossed());
    }

    #[test]
    fn test_crossword_get_x_y_width() {
        let crossword = helper_get_generic_crossword();
        assert_eq!((1, 7, 6, 5), crossword.get_x_y_width());
    }

    fn helper_get_generic_crossword() -> Crossword<'static> {
    let crossword = Crossword {
        words: vec![WordCross {word: "alpha",
                               cross: Some(CrossData {row: 6, start_point: 5, direction: Direction::Down, order: 1})},
                    WordCross {word: "bravo",
                               cross: Some(CrossData {row: 3, start_point: 6, direction: Direction::Across, order: 0})},
                    WordCross {word: "charlie",
                               cross: None},
                    WordCross {word: "delta",
                               cross: Some(CrossData {row: 1, start_point: 10, direction: Direction::Across, order: 2})
                              },
                   ]};
    return crossword;
    }

    #[test]
    fn test_get_position_end() {
        assert_eq!([12, -9],
                   get_position_end("excitement",
                                    &CrossData {row: 3, start_point: -9, direction: Direction::Across, order: 2}));
        assert_eq!([-5, 6],
                   get_position_end("hopeful",
                                    &CrossData {row: 0, start_point: -5, direction: Direction::Down, order: 8}));
    }
}

