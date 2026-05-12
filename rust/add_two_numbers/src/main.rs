use std::thread::current;

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
    let l1 = Some(Box::new(ListNode { val:2, next: Some(Box::new(ListNode { val:4, next: Some(Box::new(ListNode { val: 3, next: None })) }))}));
    let l2 = Some(Box::new(ListNode { val:5, next: Some(Box::new(ListNode { val:6, next: Some(Box::new(ListNode { val: 4, next: None })) }))}));
    let result = add_two_numbers(l1, l2);
    let mut current = &result;
    let mut vec = Vec::new();

    while let Some(node) = current{
        vec.push(node.val);
        current = &node.next;
    };

    println!("{:?}", vec);
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut l1 = &l1;
    let mut l2 = &l2;
    let mut flag:i32 = 0;
    let mut l3: Option<Box<ListNode>> = None;
    let mut value = &mut l3;

    while let (Some(n1), Some(n2)) = (l1, l2) { 
        let mut sum = n1.val + n2.val + flag;

        if sum >= 10{
            sum -= 10;
            flag = 1;
        }else{
            flag = 0;
        }

        *value = Some(Box::new(ListNode {
            val: sum,
            next: None,
        }));

        if let Some(node) = value {
            value = &mut node.next;
        }

        l1 = &n1.next;
        l2 = &n2.next;
    }

    return l3;
}