use crate::utils::append_arr;

pub fn display_number(display: &mut [String; 5], num: &str) {
    let mut counter = 0;

    for c in num.chars() {

        match c {
            ':' => display_colon(display),
            '0' => display_zero(display),
            '1' => display_one(display),
            '2' => display_two(display),
            '3' => display_three(display),
            '4' => display_four(display),
            '5' => display_five(display),
            '6' => display_six(display),
            '7' => display_seven(display),
            '8' => display_eight(display),
            '9' => display_nine(display),
            _ => (),
        }

        if counter + 1 == num.chars().count() {
            break;
        }

        display_space(display);
        counter += 1;
    }
}

fn display_space(display: &mut [String; 5]) {
    append_arr(display, "  ", 0);
    append_arr(display, "  ", 1);
    append_arr(display, "  ", 2);
    append_arr(display, "  ", 3);
    append_arr(display, "  ", 4);
}

fn display_colon(display: &mut [String; 5]) {
    append_arr(display, "  ", 0);
    append_arr(display, "██", 1);
    append_arr(display, "  ", 2);
    append_arr(display, "██", 3);
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

fn display_two(display: &mut [String; 5]) {
    append_arr(display, "██████", 0);
    append_arr(display, "    ██", 1);
    append_arr(display, "  ██  ", 2);
    append_arr(display, "██    ", 3);
    append_arr(display, "██████", 4);
}

fn display_three(display: &mut [String; 5]) {
    append_arr(display, "██████", 0);
    append_arr(display, "    ██", 1);
    append_arr(display, "  ████", 2);
    append_arr(display, "    ██", 3);
    append_arr(display, "██████", 4);
}

fn display_four(display: &mut [String; 5]) {
    append_arr(display, "██    ", 0);
    append_arr(display, "██  ██", 1);
    append_arr(display, "██████", 2);
    append_arr(display, "    ██", 3);
    append_arr(display, "    ██", 4);
}

fn display_five(display: &mut [String; 5]) {
    append_arr(display, "██████", 0);
    append_arr(display, "██    ", 1);
    append_arr(display, "██████", 2);
    append_arr(display, "    ██", 3);
    append_arr(display, "██████", 4);
}

fn display_six(display: &mut [String; 5]) {
    append_arr(display, "██    ", 0);
    append_arr(display, "██    ", 1);
    append_arr(display, "██████", 2);
    append_arr(display, "██  ██", 3);
    append_arr(display, "██████", 4);
}

fn display_seven(display: &mut [String; 5]) {
    append_arr(display, "██████", 0);
    append_arr(display, "    ██", 1);
    append_arr(display, "    ██", 2);
    append_arr(display, "    ██", 3);
    append_arr(display, "    ██", 4);
}

fn display_eight(display: &mut [String; 5]) {
    append_arr(display, "██████", 0);
    append_arr(display, "██  ██", 1);
    append_arr(display, "██████", 2);
    append_arr(display, "██  ██", 3);
    append_arr(display, "██████", 4);
}

fn display_nine(display: &mut [String; 5]) {
    append_arr(display, "██████", 0);
    append_arr(display, "██  ██", 1);
    append_arr(display, "██████", 2);
    append_arr(display, "    ██", 3);
    append_arr(display, "    ██", 4);
}
