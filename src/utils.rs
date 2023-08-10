use chrono::prelude::*;

pub fn append_arr(arr: &mut [String; 5], append: &str, pos: usize) {
    arr[pos] = String::from(arr[pos].clone() + append);
}

pub fn formatted_time() -> String {
    let time = Local::now().format("%H:%M:%S").to_string();

    return time;
}

pub fn pad_line(line: &mut String, width: &usize) {
    let padding_width: usize = ((((width - line.chars().count()) / 2) as f32).floor()) as usize;

    let mut padding_str = String::new();

    for _ in 0..padding_width {
        padding_str = String::from(padding_str + " ");
    }

    // println!("{} {} {} {}\n", i, width, arr[i].chars().count(), padding_width);

    *line = String::from(padding_str + line);
}

pub fn center_text(arr: &mut [String; 5], width: &usize, height: &usize, time: &String) -> String {
    let content: String;
    let content_height: usize;

    if width <= &arr[1].chars().count() || height <= &arr.len() {
        let mut current_time = time.clone();

        pad_line(&mut current_time, width);

        content_height = 1;
        content = current_time;
    } else {
        for i in 0..arr.len() {
            pad_line(&mut arr[i], width);
        }

        content_height = arr.len();
        content = arr.join("\n");
    }

    let top_padding_size: usize = (((height - content_height) / 2) as f32).floor() as usize;
    let bottom_padding_size: usize = height - content_height - top_padding_size as usize;

    let mut top_padding = String::new();
    let mut bottom_padding = String::new();

    for _ in 0..top_padding_size {
        top_padding = String::from(top_padding + "\n");
    }

    for _ in 0..bottom_padding_size {
        bottom_padding = String::from(bottom_padding + "\n");
    }

    // println!("{} {} {} {}", top_padding_size, bottom_padding_size, arr.len(), height);

    return String::from(top_padding + &content + &bottom_padding);
}
