use chrono::prelude::*;

pub fn append_arr(arr: &mut [String; 5], append: &str, pos: usize) {
    arr[pos] = String::from(arr[pos].clone() + append);
}

pub fn formatted_time() -> String {
    let time = Local::now().format("%H:%M:%S").to_string();

    return time;
}

pub fn center_text(arr: &mut [String; 5], width: usize, height: usize) -> String {
    // let mut centered_lines: [String; 5];

    for i in 0..arr.len() {
        let padding_width: usize = ((((width - arr[i].chars().count()) / 2) as f32).floor()) as usize;
        let mut padding_str = String::new();

        for _ in 0..padding_width {
            padding_str = String::from(padding_str + " ");
        }

        // println!("{} {} {} {}\n", i, width, arr[i].chars().count(), padding_width);

        arr[i] = String::from(padding_str + &arr[i].clone());
    }

    let top_padding_size: usize = (((height - arr.len()) / 2) as f32).floor() as usize;
    let bottom_padding_size: usize = height - arr.len() - top_padding_size as usize;

    let mut top_padding = String::new();
    let mut bottom_padding = String::new();

    for _ in 0..top_padding_size {
        top_padding = String::from(top_padding + "\n");
    }

    for _ in 0..bottom_padding_size {
        bottom_padding = String::from(bottom_padding + "\n");
    }

    // println!("{} {} {} {}", top_padding_size, bottom_padding_size, arr.len(), height);

    return String::from(top_padding + &arr.join("\n") + &bottom_padding);
}
