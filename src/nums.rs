pub fn display_number(display: &mut [String; 5], num: &str) {
    let mut counter = 0;

    for c in num.chars() {

        match c {
            '0' => display_zero(display),
            '1' => display_one(display),
            _ => (),
        }

        if counter == num.chars().count() {
            break;
        }

        display_space(display);
        counter += 1;
    }
}

fn append_arr(arr: &mut [String; 5], append: &str, pos: usize) {
    arr[pos] = String::from(arr[pos].clone() + append);
}

fn display_space(display: &mut [String; 5]) {
    append_arr(display, "  ", 0);
    append_arr(display, "  ", 1);
    append_arr(display, "  ", 2);
    append_arr(display, "  ", 3);
    append_arr(display, "  ", 4);
}

fn display_zero(display: &mut [String; 5]) {
    append_arr(display, "██████", 0);
    append_arr(display, "██  ██", 1);
    append_arr(display, "██  ██", 2);
    append_arr(display, "██  ██", 3);
    append_arr(display, "██████", 4);
}

fn display_one(display: &mut [String; 5]) {
    append_arr(display, "  ██  ", 0);
    append_arr(display, "████  ", 1);
    append_arr(display, "  ██  ", 2);
    append_arr(display, "  ██  ", 3);
    append_arr(display, "██████", 4);
}
