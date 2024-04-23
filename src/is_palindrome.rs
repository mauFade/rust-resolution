pub(crate) fn is_palindrome(str: &str) -> bool {
    let mut vec: Vec<char> = vec![];

    for ch in str.replace(" ", "").chars() {
        vec.push(ch);
    }

    let final_index = vec.len();

    let mut str1 = String::new();
    let mut str2 = String::new();

    for i in (0..final_index).rev() {
        str1.push(vec[i]);
        str2.push(vec[final_index - (i + 1)])
    }

    println!("{}, {}", str1, str2);

    str1 == str2
}
