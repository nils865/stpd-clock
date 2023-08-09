use nums::{display_zero, display_space};

mod nums;

fn main() {
    let mut display: [String; 5] = [String::from(""), String::from(""), String::from(""), String::from(""), String::from("")];

    display_zero(&mut display);
    display_space(&mut display);
    display_zero(&mut display);

    for e in &display {
        println!("{}", e);
    }
}
