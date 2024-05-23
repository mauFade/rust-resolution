pub(crate) fn accronom_arr(word: &str, arr: Vec<&str>) {
    if word.len() != arr.len() {
        return;
    }

    let mut str_chars: Vec<char> = vec![];
    let mut vec_vectors: Vec<Vec<char>> = vec![];

    for s in arr {
        let mut aux_str_chars: Vec<char> = vec![];
        for i in s.chars() {
            for lower_char in i.to_lowercase() {
                aux_str_chars.push(lower_char);
            }
        }

        vec_vectors.push(aux_str_chars);
    }

    for i in word.chars() {
        for lower_char in i.to_lowercase() {
            str_chars.push(lower_char);
        }
    }

    for i in 0..vec_vectors.len() {
        if str_chars[i] == vec_vectors[i][0] {
            println!("{}, {}", str_chars[i], vec_vectors[i][0]);
        }
    }
}
