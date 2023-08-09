use nums::display_number;
use time::formatted_time;

mod nums;
mod time;

fn main() {
    let mut display: [String; 5];

    loop {
        display = [String::from(""), String::from(""), String::from(""), String::from(""), String::from("")];

        display_number(&mut display, &formatted_time());
        
        for e in &display {
            println!("{}", e);
        }

        print!("\x1B[5A \x1B[G");
    }

}
