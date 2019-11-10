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

pub fn merge(
  mut l1: Option<Box<ListNode>>,
  mut l2: Option<Box<ListNode>>,
  mut carry: i32,
  mut result: ListNode,
) -> Option<Box<ListNode>> {
  if l1.is_none() && l2.is_none() && carry == 0 {
    return None;
  }

  if let Some(v1) = l1 {
    carry = carry + v1.val;
    l1 = v1.next;
  }

  if let Some(v2) = l2 {
    carry = carry + v2.val;
    l2 = v2.next;
  }

  result.val = carry % 10;
  result.next = merge(l1, l2, carry / 10, ListNode::new(0));

  Some(Box::new(result))
}

pub fn add_two_numbers(
  l1: Option<Box<ListNode>>,
  l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
  merge(l1, l2, 0, ListNode::new(0))
}
