use std::convert::TryFrom;

pub const X: usize = 0;
pub const Y: usize = 1;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Across,
    Down,
}

#[derive(Clone)]
pub struct CrossData {
    pub position: [i32; 2],
    pub direction: Direction,
    pub order: usize,
}

#[derive(Clone)]
pub struct WordCross<'a> {
    pub word: &'a str,
    pub cross: Option<CrossData>,
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
            if let Some(cross_data) = &word.cross {
                direction = change_direction(&cross_data.direction);
                position = cross_data.position.clone();

                for letter in word.word.chars() {
                    output.push((letter, position, direction));
                    increment_position(&cross_data.direction, &mut position);
                }
            }
        }

        return output;
    }

    pub fn get_next_order(&self) -> usize {
        let mut next_order = 0;
        for word in &self.words {
            if let Some(cross_data) = &word.cross {
                next_order = next_order.max(cross_data.order);
            }
        }
        next_order += 1;

        return next_order;
    }

    pub fn get_min_max(&self) -> (usize, usize) {
        let (_, x_width, _, y_width) = self.get_x_y_width();

        let min = x_width.min(y_width);
        let max = x_width.max(y_width);

        return (min, max);
    }

    pub fn all_words_crossed(&self) -> bool {
        let mut all_crossed = true;

        for word in &self.words {
            match word.cross {
                None => {
                    all_crossed = false;
                    break;
                }
                _ => {}
            }
        }

        return all_crossed;
    }

    pub fn print(&self) {
        let (x_low, x_width, y_low, y_width) = self.get_x_y_width();

        let mut grid = vec![vec![' '; y_width]; x_width];

        for word in &self.words {
            let mut position = [0, 0];
            let mut index = X;
            if let Some(cross_data) = &word.cross {
                position = cross_data.position.clone();

                match cross_data.direction {
                    Direction::Across => index = X,
                    Direction::Down => index = Y,
                }
            }

            for letter in word.word.chars() {
                grid[usize::try_from(position[X]-x_low).unwrap()][usize::try_from(position[Y]-y_low).unwrap()] = letter;
                position[index] += 1;
            }
        }

        for y in 0..y_width {
            for x in 0..x_width {
                print!("{}", grid[x][y]);
            }
            println!("");
        }

        println!("\n");
    }

    fn get_x_y_width(&self) -> (i32, usize, i32, usize) {
        let mut x_low = 0;
        let mut x_high = 0;
        let mut y_low = 0;
        let mut y_high = 0;

        for word in &self.words {
            if let Some(cross_data) = &word.cross {
                let position_end = get_position_end(&word.word, &cross_data);
                x_low = x_low.min(cross_data.position[X]);
                x_high = x_high.max(position_end[X]);
                y_low = y_low.min(cross_data.position[Y]);
                y_high = y_high.max(position_end[Y]);
            }
        }

        let x_width = usize::try_from(x_high - x_low).unwrap();
        let y_width = usize::try_from(y_high - y_low).unwrap();

        return (x_low, x_width, y_low, y_width);
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

fn get_position_end(word: &str, cross_data: &CrossData) -> [i32; 2] {

    let mut index = X;
    match cross_data.direction {
        Direction::Across => index = X,
        Direction::Down => index = Y,
    }

    let mut position_end = cross_data.position.clone();
    position_end[index] += word.len() as i32;

    return position_end;
}

pub fn initialise_crossword<'a>(words: &'a Vec<&str>) -> Crossword<'a> {
    let mut word_cross_vec = Vec::with_capacity(words.len());

    for index in 0..words.len() {
        let word_cross = WordCross {
            word: words[index],
            cross: None,
        };
        word_cross_vec.push(word_cross);
    }

    let first_word_cross_data = CrossData {
        position: [0, 0],
        direction: Direction::Across,
        order: 0,
    };
    word_cross_vec[0].cross = Some(first_word_cross_data);

    let crossword = Crossword {
        words: word_cross_vec,
    };

    return crossword;
}

