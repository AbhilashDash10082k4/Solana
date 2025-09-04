/*This recursove type has infinite size, Box solves this problem
    - store the pointer to the data rather than storing the data entirely
    - Box - returns a pointer to List which is of definite size independent of List size
    - impls Deref -> allows the vals to be used as references and impls Drop -> removes the var after it goes out of scope
*/
#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}
pub fn smart_pointers() {
    use List::{Cons, Nil};
    let b = Box::new(5);
    /*Box in stack and data in heap, the pointer to data in stack and data on heap */
    println!("b = {b}");
    let list = List::Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{list:?}");
}
