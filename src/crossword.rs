pub const X: usize = 0;
pub const Y: usize = 1;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Across,
    Down,
}

#[derive(Clone)]
pub struct WordCross<'a> {
    pub word: &'a str,
    pub position: Option<[i32; 2]>,
    pub direction: Option<Direction>,
    pub order: Option<usize>,
}

#[derive(Clone)]
pub struct Crossword<'a> {
    pub words: Vec<WordCross<'a>>,
}

impl Crossword<'_> {
    // todo This should return an iterator rather than a vector
    pub fn crossable_letters(&self) -> Vec<(char, [i32; 2], Direction)> {

        let mut output = Vec::new();
        let mut direction;
        let mut position;

        for word in &self.words {
            if let Some(word_direction) = &word.direction {
                if let Some(word_position) = &word.position {
                    direction = change_direction(word_direction);
                    position = word_position.clone();

                    for letter in word.word.chars() {
                        output.push((letter, position, direction));
                        increment_position(&word_direction, &mut position)
                    }
                }
            }
        }

        return output;
    }

    pub fn get_next_order(&self) -> usize {
        let mut next_order = 0;
        for word in &self.words {
            if let Some(order) = word.order {
                next_order = next_order.max(order);
            }
        }
        next_order += 1;

        return next_order;
    }

    pub fn get_min_max(&self) -> (i32, i32) {
        let mut x_low = 0;
        let mut x_high = 0;
        let mut y_low = 0;
        let mut y_high = 0;

        for word in &self.words {
            if let Some(position) = &word.position {
                x_low = x_low.min(position[X]);
                x_high = x_high.max(position[X]);
                y_low = y_low.min(position[Y]);
                y_high = y_high.max(position[Y]);
            }
        }

        let x_width = x_high - x_low;
        let y_width = y_high - y_low;

        let min = x_width.min(y_width);
        let max = x_width.max(y_width);

        return (min, max);
    }

    pub fn all_words_crossed(&self) -> bool {
        let mut all_crossed = true;

        for word in &self.words {
            if word.order == None {
                all_crossed = false;
                break;
            }
        }

        return all_crossed;
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

fn increment_position(direction: &Direction, position: &mut [i32; 2]) {
    match direction {
        Direction::Across => position[X] += 1,
        Direction::Down => position[Y] += 1,
    }
}

pub fn initialise_crossword<'a>(words: &'a Vec<&str>) -> Crossword<'a> {
    let mut word_cross_vec = Vec::with_capacity(words.len());

    for index in 0..words.len() {
        let word_cross = WordCross {
            word: words[index],
            position: None,
            direction: None,
            order: None,
        };
        word_cross_vec.push(word_cross);
    }

    word_cross_vec[0].position = Some([0, 0]);
    word_cross_vec[0].direction = Some(Direction::Across);
    word_cross_vec[0].order = Some(0);

    let crossword = Crossword {
        words: word_cross_vec,
    };

    return crossword;
}

