pub(crate) fn rotate_arr(mut nums: [i32; 7], rotate_times: usize) {
    if nums.len() == 0 {
        return;
    }

    nums.rotate_right(rotate_times);

    println!("{:?}", nums)
}
