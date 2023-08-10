use chrono::prelude::*;

pub fn append_arr(arr: &mut [String; 5], append: &str, pos: usize) {
    arr[pos] = String::from(arr[pos].clone() + append);
}

pub fn formatted_time() -> String {
    let time = Local::now().format("%H:%M:%S").to_string();

    return time;
}
