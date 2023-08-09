use chrono::prelude::*;

pub fn formatted_time() -> String {
    let time = Local::now().format("%H:%M:%S").to_string();

    return time;
}
