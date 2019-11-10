use std::cmp::{max, min};

pub struct Solution;

impl Solution {
  pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
    let len = grid.len();
    let mut max_rows = vec![0; len];
    let mut max_cols = vec![0; len];

    for (i, row) in grid.iter().enumerate() {
      for (j, &value) in row.iter().enumerate() {
        max_rows[j] = max(max_rows[j], value);
        max_cols[i] = max(max_cols[i], value);
      }
    }

    let mut result = 0;

    for (i, row) in grid.iter().enumerate() {
      for (j, &value) in row.iter().enumerate() {
        result += min(max_cols[i], max_rows[j]) - value;
      }
    }

    result
  }
}
