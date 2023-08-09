use nums::display_number;
use time::formatted_time;

mod nums;
mod time;

fn main() {
    let mut display: [String; 5] = [String::from(""), String::from(""), String::from(""), String::from(""), String::from("")];

    display_number(&mut display, &formatted_time());

    for e in &display {
        println!("{}", e);
    }
}
