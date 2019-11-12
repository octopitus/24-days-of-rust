use std::cmp::Ordering;

pub struct Solution;

impl Solution {
  pub fn char_after_space(str: &str) -> char {
    let space_pos = str.find(" ").unwrap();
    str.chars().nth(space_pos + 1).unwrap()
  }

  pub fn reorder_log_files(mut logs: Vec<String>) -> Vec<String> {
    logs.sort_by(|a, b| {
      let space_a = a.find(" ").unwrap();
      let space_b = b.find(" ").unwrap();
      let ca = a.chars().nth(space_a + 1).unwrap();
      let cb = b.chars().nth(space_b + 1).unwrap();

      if !ca.is_digit(10) && cb.is_digit(10) {
        return Ordering::Less;
      }

      if ca.is_digit(10) && !cb.is_digit(10) {
        return Ordering::Greater;
      }

      if ca.is_digit(10) && cb.is_digit(10) {
        return Ordering::Equal;
      }

      let sub_a = a.get(space_a..).unwrap();
      let sub_b = b.get(space_b..).unwrap();
      let compare_result = sub_a.cmp(sub_b);

      if compare_result == Ordering::Equal {
        let ca = a.get(..(space_a - 1)).unwrap();
        let cb = b.get(..(space_b - 1)).unwrap();

        return ca.cmp(cb);
      }

      compare_result
    });

    logs
  }
}
