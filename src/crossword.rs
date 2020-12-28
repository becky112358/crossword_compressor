pub const X: usize = 0;
pub const Y: usize = 1;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Across,
    Down,
}

pub struct WordCross<'a> {
    pub word: &'a str,
    pub location: Option<[usize; 2]>,
    pub direction: Option<Direction>,
    pub order: Option<usize>,
}

pub struct Crossword<'a> {
    pub words: Vec<WordCross<'a>>,
}

impl Crossword<'_> {
    pub fn crossable_letters(&self) -> Vec<(char, [usize; 2], Direction)> {

        let mut output = Vec::new();
        let mut direction;
        let mut location;

        for word in &self.words {
            if let Some(word_direction) = &word.direction {
                if let Some(word_location) = &word.location {
                    direction = change_direction(word_direction);
                    location = word_location.clone();

                    for letter in word.word.chars() {
                        output.push((letter, location, direction));
                        increment_location(&word_direction, &mut location)
                    }
                }
            }
        }

        return output;
    }
}

fn change_direction(input: &Direction) -> Direction {
    let output;
    match input {
        Direction::Across => output = Direction::Down,
        Direction::Down => output = Direction::Across,
    }
    return output;
}

fn increment_location(direction: &Direction, location: &mut [usize; 2]) {
    match direction {
        Direction::Across => location[X] += 1,
        Direction::Down => location[Y] += 1,
    }
}

pub fn initialise_crossword<'a>(words: &'a Vec<&str>) -> Crossword<'a> {
    let mut word_cross_vec = Vec::with_capacity(words.len());

    for index in 0..words.len() {
        let word_cross = WordCross {
            word: words[index],
            location: None,
            direction: None,
            order: None,
        };
        word_cross_vec.push(word_cross);
    }

    word_cross_vec[0].location = Some([0, 0]);
    word_cross_vec[0].direction = Some(Direction::Across);
    word_cross_vec[0].order = Some(0);

    let crossword = Crossword {
        words: word_cross_vec,
    };

    return crossword;
}

