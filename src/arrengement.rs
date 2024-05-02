use std::collections::HashMap;
use std::collections::HashSet;

pub(crate) fn arrengement(nums: Vec<i32>) {
    // eliminar numeros repetidos caso existam
    let mut unique_nums: Vec<i32> = nums
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    let mut vec_hash_map: HashMap<usize, Vec<i32>> = HashMap::new();
    let mut iterations = unique_nums.len();

    unique_nums.sort();

    for j in (0..iterations).rev() {
        let mut new_vec: Vec<i32> = vec![];
        for i in 0..iterations {
            new_vec.push(unique_nums[i]);
        }

        if iterations > 0 {
            iterations -= 1;
        }

        vec_hash_map.insert(j + 1, new_vec);
    }

    println!("{:?}", vec_hash_map);
}
