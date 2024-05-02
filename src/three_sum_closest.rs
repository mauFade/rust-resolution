pub(crate) fn three_sum_closest(nums: Vec<i32>, target: i32) {
    let mut nums = nums;
    nums.sort();

    let mut closest_sum = i32::MAX;
    let mut min_diff = i32::MAX;

    for i in 0..nums.len() {
        let mut left = i + 1;
        let mut right = nums.len() - 1;

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            let diff = (sum - target).abs();

            if diff < min_diff {
                min_diff = diff;
                closest_sum = sum;
            }

            if sum < target {
                left += 1;
            } else if sum > target {
                right -= 1;
            } else {
                println!("{}", closest_sum);
            }
        }
    }

    println!("{}", closest_sum);
}
