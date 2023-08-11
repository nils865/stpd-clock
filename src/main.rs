use std::{io::{self, Write}, thread, time::Duration};

use nums::display_number;
use term_size::dimensions;
use utils::{formatted_time, center_text};

mod nums;
mod utils;

fn output(terminal_width: &usize, terminal_height: &usize, time: &mut String, display: &mut [String; 5]) {
    *time = formatted_time();

    display_number(display, &time);
    
    let text = center_text(display, terminal_width, terminal_height, time);


    print!("{}", text);

    print!("\x1B[1;1H");
}

fn main() {
    let mut display: [String; 5];

    let mut time: String = formatted_time();

    let mut terminal_width: usize = 0;
    let mut terminal_height: usize = 0;

    let mut new_terminal_width: usize = 0;
    let mut new_terminal_height: usize = 0;

    print!("\x1B[1;1H");
    io::stdout().flush().expect("Failed to flush");

    loop {
        display = [String::from(""), String::from(""), String::from(""), String::from(""), String::from("")];

        if let Some((w, h)) = dimensions() {
            new_terminal_width = w;
            new_terminal_height = h;
        }

        if new_terminal_width != terminal_width || new_terminal_height != terminal_height {
            terminal_height = new_terminal_height;
            terminal_width = new_terminal_width;

            print!("\x1Bc");
            output(&terminal_width, &terminal_height, &mut time, &mut display);
        } else if time != formatted_time() {
            output(&terminal_width, &terminal_height, &mut time, &mut display);
        }

        thread::sleep(Duration::from_millis(10));
    }
}
