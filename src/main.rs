use nums::display_number;

mod nums;

fn main() {
    let mut display: [String; 5] = [String::from(""), String::from(""), String::from(""), String::from(""), String::from("")];

    // display_zero(&mut display);
    // display_space(&mut display);
    // display_zero(&mut display);

    display_number(&mut display, "1001");

    for e in &display {
        println!("{}", e);
    }
}
