use super::*;

#[test]
fn direction_index() {
    assert_eq!(X, Direction::Across.index());
    assert_eq!(Y, Direction::Down.index());
}

#[test]
fn direction_change() {
    assert_eq!(Direction::Down, Direction::Across.change());
    assert_eq!(Direction::Across, Direction::Down.change());
}

#[test]
fn cross_data_get_position() {
    let cross_data = CrossData {
        row: 5,
        start_point: 12,
        direction: Direction::Across,
        order: 6,
    };
    assert_eq!([12, 5], cross_data.get_position());
    let cross_data = CrossData {
        row: 28,
        start_point: -100,
        direction: Direction::Down,
        order: 98,
    };
    assert_eq!([28, -100], cross_data.get_position());
}

#[test]
fn crossword_get_crossable_letters() {
    let crossword = helper_get_generic_crossword();

    let crossable_letters = crossword.get_crossable_letters();

    assert_eq!(
        vec![
            ('a', 6, 5, Direction::Across),
            ('l', 7, 5, Direction::Across),
            ('p', 8, 5, Direction::Across),
            ('h', 9, 5, Direction::Across),
            ('a', 10, 5, Direction::Across),
            ('b', 3, 6, Direction::Down),
            ('r', 4, 6, Direction::Down),
            ('a', 5, 6, Direction::Down),
            ('v', 6, 6, Direction::Down),
            ('o', 7, 6, Direction::Down),
            ('d', 1, 10, Direction::Down),
            ('e', 2, 10, Direction::Down),
            ('l', 3, 10, Direction::Down),
            ('t', 4, 10, Direction::Down),
            ('a', 5, 10, Direction::Down),
        ],
        crossable_letters
    );
}

#[test]
fn crossword_get_next_order() {
    let crossword = helper_get_generic_crossword();
    assert_eq!(3, crossword.get_next_order());
}

#[test]
fn crossword_get_min_max() {
    let crossword = helper_get_generic_crossword();
    assert_eq!((5, 7), crossword.get_min_max());
}

#[test]
fn crossword_all_words_crossed() {
    let mut crossword = helper_get_generic_crossword();
    assert!(!crossword.all_words_crossed());

    crossword.words[1].cross = Some(CrossData {
        row: 3,
        start_point: 10,
        direction: Direction::Across,
        order: 0,
    });
    crossword.words[3].cross = Some(CrossData {
        row: 1,
        start_point: 6,
        direction: Direction::Across,
        order: 2,
    });
    crossword.words[2].cross = Some(CrossData {
        row: 0,
        start_point: 2,
        direction: Direction::Down,
        order: 3,
    });
    assert!(crossword.all_words_crossed());
}

#[test]
fn crossword_get_x_y_width() {
    let crossword = helper_get_generic_crossword();
    assert_eq!((1, 7, 6, 5), crossword.get_x_y_width());
}

fn helper_get_generic_crossword() -> Crossword<'static> {
    let crossword = Crossword {
        words: vec![
            WordCross {
                word: "alpha",
                cross: Some(CrossData {
                    row: 5,
                    start_point: 6,
                    direction: Direction::Down,
                    order: 1,
                }),
            },
            WordCross {
                word: "bravo",
                cross: Some(CrossData {
                    row: 6,
                    start_point: 3,
                    direction: Direction::Across,
                    order: 0,
                }),
            },
            WordCross {
                word: "charlie",
                cross: None,
            },
            WordCross {
                word: "delta",
                cross: Some(CrossData {
                    row: 10,
                    start_point: 1,
                    direction: Direction::Across,
                    order: 2,
                }),
            },
        ],
    };
    return crossword;
}

#[test]
fn test_get_position_end() {
    assert_eq!(
        [0, 3],
        get_position_end(
            "excitement",
            &CrossData {
                row: 3,
                start_point: -9,
                direction: Direction::Across,
                order: 2
            }
        )
    );
    assert_eq!(
        [0, 1],
        get_position_end(
            "hopeful",
            &CrossData {
                row: 0,
                start_point: -5,
                direction: Direction::Down,
                order: 8
            }
        )
    );
}
