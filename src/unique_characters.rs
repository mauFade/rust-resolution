pub(crate) fn unique_characters(string: &str) {
    let mut chars: Vec<char> = vec![];
    let mut has_repeated_chars = false;

    for i in string.chars() {
        if chars.contains(&i) == true {
            has_repeated_chars = true
        }

        chars.push(i);
    }

    if has_repeated_chars {
        println!("The string has repeated chars: {:?}", chars)
    } else {
        println!("The string dont have any repeated chars: {:?}", chars)
    }
}
