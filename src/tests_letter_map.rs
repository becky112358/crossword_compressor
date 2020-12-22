#[cfg(test)]
mod tests {
    use crate::letter_map::*;

    #[test]
    fn test_get_letter_map() {
        let words = vec![
            "the",
            "quick",
            "brown",
            "fox",
            "jumps",
            "ovEr",
            "a",
            "dog",
        ];

        let word_usages = create_word_usages(&words);

        let letter_map = get_letter_map(&word_usages);

        let i_entry = letter_map.get(&'i').unwrap();
        assert_eq!(1, i_entry.len());
        helper_assert_vector_of_word_usages_contains_word(i_entry, "quick");

        let z_entry = letter_map.get(&'z');
        assert_eq!(None, z_entry);

        let e_entry = letter_map.get(&'e').unwrap();
        assert_eq!(2, e_entry.len());
        helper_assert_vector_of_word_usages_contains_word(e_entry, "the");
        helper_assert_vector_of_word_usages_contains_word(e_entry, "ovEr");
    }

    fn helper_assert_vector_of_word_usages_contains_word(word_usages: &Vec<&WordUsage>, word: &str) {
        let mut contains_word = false;

        for word_usage in word_usages {
            if word_usage.word.eq(word) {
                contains_word = true;
                break;
            }
        }

        assert_eq!(true, contains_word);
    }
}

