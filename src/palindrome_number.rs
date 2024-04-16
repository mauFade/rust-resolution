pub(crate) fn palindrome_number(number: i32) {
    let mut chars1: Vec<char> = vec![];
    let mut chars2: Vec<char> = vec![];

    for i in number.to_string().chars() {
        chars1.push(i);
        chars2.push(i);
    }

    let is_palindrome = chars1.iter().eq(chars2.iter().rev());

    if is_palindrome {
        println!("{} é um numero palindromo", number.to_string())
    } else {
        println!("{} NÃO é um numero palindromo", number.to_string())
    }
}
