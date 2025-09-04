/*This recursove type has infinite size, Box solves this problem
    - store the pointer to the data rather than storing the data entirely
    - Box - returns a pointer to List which is of definite size independent of List size
    - impls Deref -> allows the vals to be used as references and impls Drop -> removes the var after it goes out of scope
*/
use std::{ops::Deref, mem::drop};

#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

//custom smart pointer
pub struct CustomSP<T>(T);
impl<T> CustomSP<T> {
    fn new(x: T) -> CustomSP<T> {
        CustomSP(x)
    }
}
impl<T> Deref for CustomSP<T> {
    type Target = T;
    /* .0 returns the 1st elem of th etuple struct*/
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> Drop for CustomSP<T> {
    fn drop (&mut self) {
        println!("CustomSp dropped");
        //will run when the struct will be dropped
    }
}
pub fn smart_pointers() {
    use List::{Cons, Nil};
    /*Box in stack and data in heap, the pointer to data in stack and data on heap */
    let list = List::Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{list:?}");
    
    let x = 5;
    let y = Box::new(x);
    let z = CustomSP::new(x);
    /*y has copied value of x in Box rather than having a reference to x,
    z = uses CustomSP which returns a reference to inner val on dereferencing
    */
    assert_eq!(5, *y);
    assert_eq!(5, x);
    assert_eq!(5, *z);

    /*Deref coercion - 
    Rust calls deref again to convert &String to &str which is the param type of hello
    1st call -> &CustomSP<Stirng> to &String
    2nd call -> &String to &str

    Rust can call Dref::deref any no. of times to convert the type of data to match param type. This (the no. of calls) is decided at compile time so there is no load on runtime

    Deref trait can be also used with mutable vals. The immutable vals cannot be converted to mutable references but the reverse can happen
    */
    drop(x); //drops the data passed before going out of scope
    fn hello(s: &str) {
        println!("{s}");
    }
    let s = CustomSP::new(String::from("ad"));
    hello(&s);
}
