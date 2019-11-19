use std::cmp::max;

impl Solution {
  pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut sum = i32::min_value();
    let mut temp = 0;

    for i in 0..nums.len() {
      temp = max(nums[i], nums[i] + temp);

      if temp > sum {
        sum = temp;
      }
    }

    sum
  }
}
