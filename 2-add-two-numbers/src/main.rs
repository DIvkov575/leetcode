#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

fn main() {
    let a = Some(Box::new(ListNode {val: 5, next: Some(Box::new(ListNode::new(3)))}));
    let b = Some(Box::new(ListNode {val: 9, next: Some(Box::new(ListNode::new(1)))}));

    let a = add_two_numbers(a, b);
    println!("{:?}", a);

}


pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    return recurese(l1, l2, 0);

    fn recurese(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, icarry: i32) -> Option<Box<ListNode>> {
        let mut node1: Option<Box<ListNode>>;
        let mut node2: Option<Box<ListNode>>;
        let mut sum: i32 = 0;
        let mut a;
        let mut b;
        let mut ocarry;

        if *&l1.is_none() && *&l2.is_none() {
            if icarry == 1 {
                return Some(Box::new(ListNode { val: 1, next: None }));
            } else {
                return None;
            }
        }

        if let Some(node) = l1 {
            a = node.val;
            node1 = node.next;
        } else {
            a = 0;
            node1 = None;
        }
        if let Some(node) = l2 {
            b = node.val;
            node2 = node.next;
        } else {
            b = 0;
            node2 = None;
        }

        ocarry = (a + b + icarry) / 10;
        sum = (a + b + icarry) % 10;



        return Some(Box::new(ListNode { val: sum, next: recurese(node1, node2, ocarry) }));

    }


}