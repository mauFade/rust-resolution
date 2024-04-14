pub(crate) fn one_edit_away(string1: &str, string2: &str) {
    if (string1.chars().count() as i32 - string1.chars().count() as i32).abs() > 1 {
        println!("Strings are not one edit away");

        return;
    }

    let mut edits = 0;
    let mut i = 0;
    let mut j = 0;

    while i < string1.chars().count() && j < string2.chars().count() {
        if string1.chars().nth(i) != string2.chars().nth(j) {
            if edits == 1 {
                println!("Strings are not one edit away");
            }

            if string1.chars().count() > string2.chars().count() {
                i += 1;
            } else if string1.chars().count() < string2.chars().count() {
                j += 1;
            } else {
                i += 1;
                j += 1;
            }
            edits += 1;
        } else {
            i += 1;
            j += 1;
        }
    }

    if i < string1.chars().count() || j < string2.chars().count() {
        edits += 1;
    }

    if edits == 1 {
        println!("Strings are one edit away")
    }
}
