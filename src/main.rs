use nums::display_number;
use utils::formatted_time;

mod nums;
mod utils;

fn main() {
    let mut display: [String; 5];

    let mut time: String = formatted_time();

    loop {
        display = [String::from(""), String::from(""), String::from(""), String::from(""), String::from("")];

        if time != formatted_time() {
            time = formatted_time();

            display_number(&mut display, &time);
            
            for e in &display {
                println!("{}", e);
            }
    
            print!("\x1B[5A \x1B[G");
        }
    }

}
