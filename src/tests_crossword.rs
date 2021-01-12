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
    fn test_crossword_crossable_letters() {
        let crossword = helper_get_generic_crossword();

        let crossable_letters = crossword.crossable_letters();

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
                       ],
                   crossable_letters);
    }

    #[test]
    fn test_crossword_get_next_order() {
        let crossword = helper_get_generic_crossword();
        assert_eq!(2, crossword.get_next_order());
    }

    fn helper_get_generic_crossword() -> Crossword<'static> {
    let crossword = Crossword {
            words: vec![WordCross {word: "alpha",
                                   cross: Some(CrossData {position: [5, 6], direction: Direction::Down, order: 1})},
                        WordCross {word: "bravo",
                                   cross: Some(CrossData {position: [3, 6], direction: Direction::Across, order: 0})},
                        WordCross {word: "charlie",
                                   cross: None},
                       ]};
    return crossword;
    }
}

