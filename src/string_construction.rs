pub(crate) fn string_construction() {
    // str1 = ramsomNote, str2 = magazine
    let str1 = "ramsomNote";
    let str2 = "magazine";
    let mut vec: Vec<char> = vec![];
    let mut can_be_constructed = true;

    for ch in str2.to_lowercase().chars() {
        vec.push(ch)
    }

    for ch in str1.to_lowercase().chars() {
        if !vec.contains(&ch) {
            can_be_constructed = false;
        }
    }

    if can_be_constructed {
        println!("{} pode ser construída usando os chars de {}", str1, str2);
    } else {
        println!(
            "{} NÃO pode ser construída usando os chars de {}",
            str1, str2
        );
    }
}
