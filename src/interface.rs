


// Definition for singly-linked list.
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

//This will implement the shape of the object --> Using struct
pub struct  Algorithm {}


//This will implement what are the functions should be included with the object// --> Using impl
impl  Algorithm {
    
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(mut slow) => {
                let mut fast = slow.clone();
                while fast.next.is_some() && fast.next.as_ref().unwrap().next.is_some() {
                    slow = slow.next.unwrap();
                    fast = fast.next.unwrap().next.unwrap();
                }
                Some(slow.next)
            }
            None => None,
        }
    }
}