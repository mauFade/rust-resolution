pub(crate) fn is_anagram(str1: &str, str2: &str) -> bool {
    let mut vec1: Vec<char> = vec![];
    let mut vec2: Vec<char> = vec![];

    for ch in str1.chars() {
        if ch != ' ' {
            vec1.push(ch);
        }
    }

    for ch in str2.chars() {
        if ch != ' ' {
            vec2.push(ch);
        }
    }

    vec1.sort();
    vec2.sort();

    vec1 == vec2
}
