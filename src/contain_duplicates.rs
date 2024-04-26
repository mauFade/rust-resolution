use ::std::collections::HashMap;

pub(crate) fn contain_duplicates(vec: Vec<i32>) {
    let mut verify: HashMap<&i32, bool> = HashMap::new();
    let mut has_duplicates = false;

    for i in vec.iter() {
        if verify.get(i) == Some(&true) {
            has_duplicates = true;
        }

        verify.insert(i, true);
    }

    println!("{:?}, tem duplicatas: {}", vec, has_duplicates)
}
