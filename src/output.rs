
const SCREEN_WIDTH: usize = 80;

pub fn output_clear_message(message: &str) {
    output_stars();
    println!("{}", message);
    output_stars();
    println!("");
}

fn output_stars() {
    for _ in 0..SCREEN_WIDTH {
        print!("*");
    }
    println!();
}

