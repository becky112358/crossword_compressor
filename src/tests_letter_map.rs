#[cfg(test)]
mod tests {
    use crate::letter_map::*;

    #[test]
    fn test_get_letter_map() {
        let words = vec![
            "unlisted",
            "the",
            "quick",
            "brown",
            "fox",
            "jumps",
            "ovEr",
            "a",
            "dog",
        ];

        let letter_map = get_letter_map(&words);

        let i_entry = letter_map.get(&'i').unwrap();
        assert_eq!(1, i_entry.len());
        helper_word_and_letter_vector_contains_word_index(&i_entry, 2);

        if let Some(z_entry) = letter_map.get(&'z') {
            assert!(false, "No words contain letter z, but the letter map contains a z entry.");
        }

        let e_entry = letter_map.get(&'e').unwrap();
        assert_eq!(2, e_entry.len());
        helper_word_and_letter_vector_contains_word_index(&e_entry, 1);
        helper_word_and_letter_vector_contains_word_index(&e_entry, 6);
    }

    fn helper_word_and_letter_vector_contains_word_index(word_and_letters: &Vec<WordAndLetter>, word_index: usize) {
        let mut contains = false;

        for word_and_letter in word_and_letters {
            if word_and_letter.word_index == word_index {
                contains = true;
                break;
            }
        }

        assert_eq!(true, contains);
    }
}

