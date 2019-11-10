impl Solution {
  pub fn starts_with_digit(str: &str) -> bool {
    let first_char = str.chars().nth(0);

    if let Some(c) = first_char {
      return c.is_digit(10);
    }

    false
  }

  pub fn my_atoi(str: String) -> i32 {
    let s = str.trim();
    let is_negative = s.starts_with('-');
    let sign = if is_negative { -1 } else { 1 };
    let s = if is_negative || s.starts_with('+') {
      &s[1..]
    } else {
      s
    };

    if !Solution::starts_with_digit(s) {
      return 0;
    }

    let max = i32::max_value() as i64;
    let min = max + 1;

    let mut chars = s.chars();
    let mut result: i64 = 0;

    while let Some(c) = chars.next() {
      if let Some(digit) = c.to_digit(10) {
        result = result * 10 + (digit as i64);
        if !is_negative && result > max {
          result = max;
          break;
        } else if is_negative && result > min {
          result = min;
          break;
        };
      } else {
        break;
      }
    }

    (result * sign) as i32
  }
}
