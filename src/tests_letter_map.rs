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

        let letter_map = get_letter_map(&words);

        let i_entry = letter_map.get(&'i').unwrap();
        assert_eq!(1, i_entry.len());
        assert!(i_entry.contains(&1));

        let z_entry = letter_map.get(&'z');
        assert_eq!(None, z_entry);

        let e_entry = letter_map.get(&'e').unwrap();
        assert_eq!(2, e_entry.len());
        assert!(e_entry.contains(&0));
        assert!(e_entry.contains(&5));
    }
}

