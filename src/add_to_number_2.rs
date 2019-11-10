#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }
}

pub struct Solution;

impl Solution {
  pub fn get_len(l: &Option<Box<ListNode>>) -> i32 {
    let mut len = 0;
    let mut cur = l;

    while let Some(v) = cur {
      len = len + 1;
      cur = &v.next;
    }

    len
  }

  pub fn add_leading_zeros(l: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut i = 0;
    let mut cur = l;

    while i < n {
      let mut n = ListNode::new(0);
      n.next = cur;
      cur = Some(Box::new(n));
      i = i + 1;
    }

    cur
  }

  pub fn merge(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    mut result: ListNode,
  ) -> (i32, Option<Box<ListNode>>) {
    if let (Some(v1), Some(v2)) = (l1, l2) {
      let (carry, next) = Solution::merge(v1.next, v2.next, ListNode::new(0));
      let value = v1.val + v2.val + carry;
      result.val = value % 10;
      result.next = next;
      (value / 10, Some(Box::new(result)))
    } else {
      return (0, None);
    }
  }

  pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    let len1 = Solution::get_len(&l1);
    let len2 = Solution::get_len(&l2);

    let (l1, l2) = if len1 > len2 {
      let new_l2 = Solution::add_leading_zeros(l2, len1 - len2);
      (l1, new_l2)
    } else if len2 > len1 {
      let new_l1 = Solution::add_leading_zeros(l1, len2 - len1);
      (new_l1, l2)
    } else {
      (l1, l2)
    };

    let (carry, result) = Solution::merge(l1, l2, ListNode::new(0));

    if carry > 0 {
      let mut n = ListNode::new(carry);
      n.next = result;
      return Some(Box::new(n));
    }

    return result;
  }
}
