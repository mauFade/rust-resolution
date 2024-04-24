pub(crate) fn move_zeros(mut vec: Vec<i32>) {
    let mut indexes: Vec<usize> = vec![];
    let mut removed_zeros: i32 = 0;

    println!("{:?}", vec);

    for index in (0..vec.len()).rev() {
        if vec[index] == 0 {
            indexes.push(index);
            removed_zeros += 1;
        }
    }

    for &i in indexes.iter() {
        vec.remove(i);
    }

    for _ in 0..removed_zeros {
        vec.push(0);
    }

    println!("{:?}", vec);
}
