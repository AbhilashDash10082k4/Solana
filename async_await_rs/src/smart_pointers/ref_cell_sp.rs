/* Borrowing rules are enforced for -
    - Box<T> at compile time -so, , if not followed, code will give compiler error -which is best as every error is inspected before hand
    - RefCell<T> at runtime -so, if not followed, code will panic
    - RefCell<T> allows immutable/mutable borrowing at runtime
    - Box<T> allows immutable/mutable borrowing at compile time
    - Rc<T> allows immutable borrowing at compile time
*/
/* impl RefCell<T> for mutable references at runtime
- Creating a mock obj that will keep track of msgs to send rate limiting msgs
*/
use self::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

pub trait Messenger {
    fn send(&self, msg: &str);
}
pub struct Limit<'a, T: Messenger> {
    msngr: &'a T,
    max: usize,
    curr_val: usize,
}
impl<'a, T> Limit<'a, T>
where
    T: Messenger,
{
    pub fn new(msngr: &'a T, max: usize) -> Limit<'a, T> {
        Limit {
            msngr,
            curr_val: 0,
            max,
        }
    }
    pub fn set_val(&mut self, curr_val: usize) {
        self.curr_val = curr_val;
        let percentage = self.curr_val as f64 / self.max as f64;
        if percentage >= 1.0 {
            self.msngr.send("Exceeded the limit");
        } else if percentage >= 0.75 {
            self.msngr.send("75% of the limit reached");
        } else if percentage >= 0.5 {
            self.msngr.send("50% of the limit reached");
        }
    }
}

//inline test for the above
#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;
    pub struct MockStr {
        sent_msgs: RefCell<Vec<String>>,
    }
    impl MockStr {
        fn new() -> MockStr {
            MockStr {
                sent_msgs: RefCell::new(vec![]),
            }
        }
    }
    impl Messenger for MockStr {
        fn send(&self, msg: &str) {
            self.sent_msgs.borrow_mut().push(String::from(msg));
            /* borrow_mut muably borrows the inner val and panics if the val is already borrowed
            - returns RefMut<T> -mutable-> this impls Deref
            */
        }
    }
    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockStr::new();
        //mock_messenger is same type as of Limit as it impls Messenger trait

        let mut limit_tracker = Limit::new(&mock_messenger, 100);

        limit_tracker.set_val(80);

        assert_eq!(mock_messenger.sent_msgs.borrow().len(), 1);
        /*borrow()- Immutably borrows the wrapped value.
        - this returns Ref<T> -immutable -> this impls Deref
        */
        /*Keeps track of the no. of Ref<T> and RefMut<T> are called and inc/dec the counts of these when they come into /go out of the scope  */
    }
}
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn ref_cell_sp() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}
