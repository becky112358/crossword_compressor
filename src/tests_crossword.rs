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

        assert_eq!(vec![('a', [5, 6], Direction::Across),
                        ('l', [5, 7], Direction::Across),
                        ('p', [5, 8], Direction::Across),
                        ('h', [5, 9], Direction::Across),
                        ('a', [5, 10], Direction::Across),
                        ('b', [3, 6], Direction::Down),
                        ('r', [4, 6], Direction::Down),
                        ('a', [5, 6], Direction::Down),
                        ('v', [6, 6], Direction::Down),
                        ('o', [7, 6], Direction::Down),
                        ('d', [1, 10], Direction::Down),
                        ('e', [2, 10], Direction::Down),
                        ('l', [3, 10], Direction::Down),
                        ('t', [4, 10], Direction::Down),
                        ('a', [5, 10], Direction::Down),
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

        crossword.words[1].cross = Some(CrossData {position: [3, 10], direction: Direction::Across, order: 0});
        crossword.words[3].cross = Some(CrossData {position: [1, 6], direction: Direction::Across, order: 2});
        crossword.words[2].cross = Some(CrossData {position: [2, 0], direction: Direction::Down, order: 3});
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
                                   cross: Some(CrossData {position: [5, 6], direction: Direction::Down, order: 1})},
                        WordCross {word: "bravo",
                                   cross: Some(CrossData {position: [3, 6], direction: Direction::Across, order: 0})},
                        WordCross {word: "charlie",
                                   cross: None},
                        WordCross {word: "delta",
                                   cross: Some(CrossData {position: [1, 10], direction: Direction::Across, order: 2})},
                       ]};
    return crossword;
    }

    #[test]
    fn test_get_position_end() {
        assert_eq!([12, -9],
                   get_position_end("excitement",
                                    &CrossData {position: [3, -9], direction: Direction::Across, order: 2}));
        assert_eq!([-5, 6],
                   get_position_end("hopeful", &CrossData {position: [-5, 0], direction: Direction::Down, order: 8}));
    }
}

