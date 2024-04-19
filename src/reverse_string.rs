pub(crate) fn reverse_string(str: String) {
    let mut initial_chars: Vec<char> = vec![];

    for ch in str.chars() {
        initial_chars.push(ch);
    }

    let mut final_str = String::new();

    initial_chars.reverse();

    for ch in initial_chars {
        final_str.push(ch);
    }

    println!("{}, {}", str, final_str);
}
