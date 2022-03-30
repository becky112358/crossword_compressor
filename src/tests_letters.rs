#[cfg(test)]
mod tests {
    use crate::letters::*;

    #[test]
    fn test_letters_to_lowercase() {
        let words_input = vec!["ALL_UPPERCASE", "mIXeD casE", "all_lowercase"];

        let words_output = letters_to_lowercase(&words_input);

        assert_eq!("all_uppercase".to_string(), words_output[0]);
        assert_eq!("mixed case".to_string(), words_output[1]);
        assert_eq!("all_lowercase".to_string(), words_output[2]);
    }

    #[test]
    fn test_letters_get_map() {
        let words = vec![
            "the".to_string(),
            "quick".to_string(),
            "brown".to_string(),
            "fox".to_string(),
            "jumps".to_string(),
            "over".to_string(),
            "a".to_string(),
            "dog".to_string(),
        ];

        let letter_map = letters_get_map(&words);

        let i_entry = letter_map.get(&'i').unwrap();
        assert_eq!(1, i_entry.len());
        helper_word_and_letter_vector_contains_word_index(&i_entry, 1);

        if let Some(_) = letter_map.get(&'z') {
            assert!(
                false,
                "No words contain letter z, but the letter map contains a z entry."
            );
        }

        let e_entry = letter_map.get(&'e').unwrap();
        assert_eq!(2, e_entry.len());
        helper_word_and_letter_vector_contains_word_index(&e_entry, 0);
        helper_word_and_letter_vector_contains_word_index(&e_entry, 5);
    }

    fn helper_word_and_letter_vector_contains_word_index(
        word_and_letters: &Vec<WordAndLetter>,
        word_index: usize,
    ) {
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
