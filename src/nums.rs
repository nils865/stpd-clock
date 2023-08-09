pub fn append_arr(arr: &mut [String; 5], append: &str, pos: usize) {
    arr[pos] = String::from(arr[pos].clone() + append);
}

pub fn display_space(display: &mut [String; 5]) {
    append_arr(display, "  ", 0);
    append_arr(display, "  ", 1);
    append_arr(display, "  ", 2);
    append_arr(display, "  ", 3);
    append_arr(display, "  ", 4);
}

pub fn display_zero(display: &mut [String; 5]) {
    append_arr(display, "██████", 0);
    append_arr(display, "██  ██", 1);
    append_arr(display, "██  ██", 2);
    append_arr(display, "██  ██", 3);
    append_arr(display, "██████", 4);
}
