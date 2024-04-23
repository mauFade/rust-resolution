pub(crate) fn rotate_arr(mut nums: [i32; 7], rotate_times: usize) {
    nums.rotate_right(rotate_times);

    println!("{:?}", nums)
}
