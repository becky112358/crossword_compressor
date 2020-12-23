
use std::collections::HashMap;

pub fn get_letter_map(words: &Vec<&str>) -> HashMap<char, Vec<usize>> {
    let mut letter_map: HashMap<char, Vec<usize>> = HashMap::new();

    for i in 0..words.len() {
        for letter in words[i].to_lowercase().chars() {
            let words_with_letter = letter_map.entry(letter).or_insert(vec![i]);
            if !words_with_letter.contains(&i) {
                words_with_letter.push(i);
            }
        }
    }

    return letter_map;
}


#[cfg(test)]
#[path = "./tests_letter_map.rs"]
mod tests_letter_map;


