#[derive(Debug)]
struct CharCount {
    letter: char,
    count: usize,
}

pub(crate) fn string_compression(string: &str) {
    let mut chars: Vec<char> = vec![];
    let mut chars2: Vec<char> = vec![];
    let mut repeated_chars: Vec<char> = vec![];
    let mut initial_string = String::new();
    let mut counts: Vec<CharCount> = vec![];

    for char in string.chars() {
        chars.push(char);
        chars2.push(char)
    }

    for i in 1..chars.len() {
        if chars[i] == chars[i - 1] {
            repeated_chars.push(chars[i]);
        }
    }

    for char in chars2 {
        if !repeated_chars.contains(&char) {
            initial_string.push(char)
        }
    }

    for char in repeated_chars {
        if let Some(count_struct) = counts.iter_mut().find(|c| c.letter == char) {
            count_struct.count += 1;
        } else {
            counts.push(CharCount {
                count: 1,
                letter: char,
            });
        }
    }

    let mut final_string = String::from(initial_string);

    for item in counts {
        if chars.contains(&item.letter) {
            let real_count = item.count + 1 as usize;

            final_string.push(item.letter);
            final_string.push_str(&real_count.to_string())
        }
    }

    println!(
        "\nA string {} pode ser abreviada como {}",
        string, final_string
    )
}
