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
}

