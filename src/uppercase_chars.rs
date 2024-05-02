pub(crate) fn uppercase_chars(data_string: &str) {
    let mut count = 0;

    for ch in data_string.chars() {
        if ch.is_ascii_uppercase() {
            count += 1;
        }
    }

    println!("{} tem {} letras maiúsculas", data_string, count);
}
