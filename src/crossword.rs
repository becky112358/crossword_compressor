use std::convert::TryFrom;

const X: usize = 0;
const Y: usize = 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Across,
    Down,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CrossData {
    pub row: i32,
    pub start_point: i32,
    pub direction: Direction,
    pub order: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WordCross<'a> {
    pub word: &'a str,
    pub cross: Option<CrossData>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Crossword<'a> {
    pub words: Vec<WordCross<'a>>,
}

impl Direction {
    pub fn index(&self) -> usize {
        match self {
            Direction::Across => X,
            Direction::Down => Y,
        }
    }

    pub fn change(&self) -> Direction {
        match self {
            Direction::Across => Direction::Down,
            Direction::Down => Direction::Across,
        }
    }
}

impl CrossData {
    fn get_position(&self) -> [i32; 2] {
        match self.direction {
            Direction::Across => [self.start_point, self.row],
            Direction::Down => [self.row, self.start_point],
        }
    }
}

impl Crossword<'_> {
    pub fn get_crossable_letters(&self) -> Vec<(char, i32, i32, Direction)> {
        let mut output = Vec::new();

        for word in &self.words {
            if let Some(cross_data) = &word.cross {
                let direction = cross_data.direction.change();
                let mut row = cross_data.start_point;
                let mid_point = cross_data.row;

                for letter in word.word.chars() {
                    output.push((letter, row, mid_point, direction));
                    row += 1;
                }
            }
        }

        output
    }

    pub fn get_next_order(&self) -> usize {
        let mut next_order = 0;
        for word in &self.words {
            if let Some(cross_data) = &word.cross {
                next_order = next_order.max(cross_data.order);
            }
        }
        next_order += 1;

        next_order
    }

    pub fn get_min_max(&self) -> (usize, usize) {
        let (_, x_width, _, y_width) = self.get_x_y_width();

        let min = x_width.min(y_width);
        let max = x_width.max(y_width);

        (min, max)
    }

    pub fn all_words_crossed(&self) -> bool {
        for word in &self.words {
            if word.cross.is_none() {
                return false;
            }
        }

        true
    }

    #[allow(clippy::needless_range_loop)]
    pub fn print(&self) {
        let (x_low, x_width, y_low, y_width) = self.get_x_y_width();

        let mut grid = vec![vec![' '; y_width + 1]; x_width + 1];

        for word in &self.words {
            if let Some(cross_data) = &word.cross {
                let mut position = cross_data.get_position();
                let index = cross_data.direction.index();

                for lr in word.word.chars() {
                    grid[usize::try_from(position[X] - x_low).unwrap()]
                        [usize::try_from(position[Y] - y_low).unwrap()] = lr;
                    position[index] += 1;
                }
            }
        }

        for y in 0..y_width {
            for x in 0..x_width {
                print!("{}", grid[x][y]);
            }
            println!();
        }

        println!("\n");
    }

    fn get_x_y_width(&self) -> (i32, usize, i32, usize) {
        let mut first_word = true;

        let mut x_low = 0;
        let mut x_high = 0;
        let mut y_low = 0;
        let mut y_high = 0;

        for word in &self.words {
            if let Some(cross_data) = &word.cross {
                let position_start = cross_data.get_position();
                let position_end = get_position_end(word.word, cross_data);

                if first_word {
                    x_low = position_start[X];
                    x_high = position_end[X];
                    y_low = position_start[Y];
                    y_high = position_end[Y];
                    first_word = false;
                } else {
                    x_low = x_low.min(position_start[X]);
                    x_high = x_high.max(position_end[X]);
                    y_low = y_low.min(position_start[Y]);
                    y_high = y_high.max(position_end[Y]);
                }
            }
        }

        let x_width = usize::try_from(x_high - x_low + 1).unwrap();
        let y_width = usize::try_from(y_high - y_low + 1).unwrap();

        (x_low, x_width, y_low, y_width)
    }
}

fn get_position_end(word: &str, cross_data: &CrossData) -> [i32; 2] {
    let mut position_end = cross_data.get_position();
    let index = cross_data.direction.index();
    position_end[index] += word.len() as i32 - 1;

    position_end
}

pub fn crossword_initialise(words: &[String]) -> Crossword {
    let mut word_cross_vec = Vec::with_capacity(words.len());

    for word in words {
        let word_cross = WordCross { word, cross: None };
        word_cross_vec.push(word_cross);
    }

    let first_word_cross_data = CrossData {
        row: 0,
        start_point: 0,
        direction: Direction::Across,
        order: 0,
    };
    word_cross_vec[0].cross = Some(first_word_cross_data);

    Crossword {
        words: word_cross_vec,
    }
}

#[cfg(test)]
#[path = "./tests_crossword.rs"]
mod tests_crossword;
