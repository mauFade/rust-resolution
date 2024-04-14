pub(crate) fn retroactive_string(string1: &str, string2: &str) {
    let mut vec1: Vec<char> = vec![];
    let mut vec2: Vec<char> = vec![];
    let mut is_retroactive = true;

    for i in string1.chars() {
        vec1.push(i)
    }

    for i in string2.chars() {
        vec2.push(i)
    }

    let mut count = 0;

    if string1.len() > string2.len() {
        for &caractere in &vec1 {
            if vec2.contains(&caractere) {
                count = count + 1;
            }

            if count != vec2.len() {
                is_retroactive = false;
            }
        }
    } else {
        for &caractere in &vec2 {
            if vec1.contains(&caractere) {
                count = count + 1;
            }
        }

        if count != vec1.len() {
            is_retroactive = false;
        }
    }

    if is_retroactive {
        println!(
            "{:?}, {:?}, as strings são permutação uma da outra",
            vec1, vec2
        )
    } else {
        println!(
            "{:?}, {:?}, as strings NÃO são permutação uma da outra",
            vec1, vec2
        )
    }
}
