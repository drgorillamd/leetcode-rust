// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

fn add_two_numbers_inner(l1: Option<&Box<ListNode>>, l2: Option<&Box<ListNode>>, carry: i32) -> Option<Box<ListNode>> {
    
    // Match l1, l2, carry, add 0 if one is empty
    // make the sum % 10 and the reminder is the new carry
    
        match (l1, l2, carry) {
            // Both node have a value -> sum them % 10 and carry
            (Some(l1), Some(l2), carry) => {
            let sum = l1.val + l2.val + carry;
            let new_carry = sum / 10;
            let new_val = sum % 10;
            Some(Box::new(ListNode { val: new_val, next: add_two_numbers_inner(l1.next.as_ref(), l2.next.as_ref(), new_carry)}))
        }
        (None, Some(l), carry) | (Some(l), None, carry) => {
            let sum = l.val + carry;
            let new_carry = sum / 10;
            let new_val = sum % 10;
            Some(Box::new(ListNode { val: new_val, next: add_two_numbers_inner(l.next.as_ref(), None, new_carry)}))
        }
        (None, None, carry) => {
            if carry > 0 {
                Some(Box::new(ListNode { val: carry, next: None }))
            } else {
                None
            }
        }
    }
}
    
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    add_two_numbers_inner(l1.as_ref(), l2.as_ref(), 0)
}

#[test]
fn test_one() {
    let l1 = Some(Box::new(ListNode { val: 2, next: Some(Box::new(ListNode { val: 4, next: Some(Box::new(ListNode { val: 3, next: None }))}))}));
    let l2 = Some(Box::new(ListNode { val: 5, next: Some(Box::new(ListNode { val: 6, next: Some(Box::new(ListNode { val: 4, next: None }))}))}));
    let expected = Some(Box::new(ListNode { val: 7, next: Some(Box::new(ListNode { val: 0, next: Some(Box::new(ListNode { val: 8, next: None }))}))}));
    assert_eq!(add_two_numbers(l1, l2), expected);
}

#[test]
fn test_two() {
    let l1 = Some(Box::new(ListNode { val: 0, next: None }));
    let l2 = Some(Box::new(ListNode { val: 0, next: None }));
    let expected = Some(Box::new(ListNode { val: 0, next: None }));
    assert_eq!(add_two_numbers(l1, l2), expected);
}