pub(crate) fn kggest_num(mut nums: Vec<i32>, k: usize) {
    nums.sort();

    println!("{:?}, O {}º maior número: {}", nums, k, nums[k - 1]);
}
