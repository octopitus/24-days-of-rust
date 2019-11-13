pub struct Solution;

impl Solution {
  pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    if !word_list.contains(&end_word) {
      return 0;
    }

    let mut begin: HashSet<String> = HashSet::new();
    let mut end: HashSet<String> = HashSet::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut len = 1;

    begin.insert(begin_word.clone());
    end.insert(end_word.clone());

    while !begin.is_empty() && !end.is_empty() {
      if begin.len() > end.len() {
        let tmp = begin;
        begin = end;
        end = tmp;
      }

      let mut temp: HashSet<String> = HashSet::new();

      for word in begin.iter() {
        let mut chars: Vec<char> = word.chars().collect();

        for i in 0..chars.len() {
          for c in b'a'..b'z' {
            let old = chars[i];
            chars[i] = char::from_u32(c.into()).unwrap();
            let new_word: String = chars.iter().collect();

            if end.contains(&new_word) {
              return len + 1;
            }

            if !visited.contains(&new_word) && word_list.contains(&new_word) {
              visited.insert(new_word.clone());
              temp.insert(new_word.clone());
            }

            chars[i] = old;
          }
        }
      }

      begin = temp;
      len += 1;
    }

    0
  }
}
