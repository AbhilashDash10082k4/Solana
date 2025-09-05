/*Rc<T> -
- stands for reference counting
- tracks the no of owners of a single data
- works in single thread
- Rc::clone increases the no. of references and does not deep copy teh entire data
*/
use std::rc::Rc;
#[derive(Debug)]
enum NestedList {
    Cons(i32, Rc<NestedList>),
    Nil,
}
use self::NestedList::{Cons, Nil};

pub fn ref_count() {
    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(7, Rc::new(Nil)))))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Rc::new(Cons(7, Rc::clone(&a)));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Rc::new(Cons(7, Rc::clone(&a)));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a)); //this reduces by 1 after c goes out of scope
    println!("{b:?}");
}
