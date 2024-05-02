pub(crate) fn vec_intersection(vec1: Vec<i32>, vec2: Vec<i32>) {
    let mut final_vec: Vec<i32> = vec![];

    for int1 in &vec1 {
        for int2 in &vec2 {
            if int1 == int2 && !final_vec.contains(int1) {
                final_vec.push(*int1);
            }
        }
    }

    println!("{:?}", final_vec);
}
