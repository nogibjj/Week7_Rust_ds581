pub fn is_valid(input: &str) -> bool {
    // Initialize a counter for open brackets
    let mut open_brackets = 0;

    // Check whether the string has an equal number of open and closed brackets
    let mut valid_string = true;
    for c in input.chars() {
        match c {
            '(' => open_brackets += 1,
            ')' => {
                if open_brackets == 0 {
                    valid_string = false;
                    break;
                } else {
                    open_brackets -= 1;
                }
            },
            _ => continue,
        }
    }

    valid_string && open_brackets == 0
}

