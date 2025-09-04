use std::{
    sync::{Arc, Mutex, mpsc},
    thread::{self},
    time::Duration,
};
pub mod print_type_of;
pub mod smart_pointers;
use self::print_type_of::print_type_of_var;
// use self::smart_pointers::smart_pointers;
//mpsc - multiple producer and single consumer
fn main() {
    //making a var and moving it to the thread closure(the callback fn) using move
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("{v:?}"); //moved the ownership of v to the thread
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1)); //thread::sleep force a thread to stop its execution for a short duration, allowing a different thread to run. 
        }
    });
    handle.join().unwrap(); //.join blocks 1 thread until the other thread(repersented by handle) finishes executing, here, in this line, the main thread waits until all the handle threads finish executing and then it runs
    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
    //if handle.join().unwrap() was called here , then the main thread would have executed but would have ended only after the spawned threads finished running

    //channel for communication-
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone(); //multiple transmiters single consumer
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        // let l = String::from("AD");
        let v = vec![
            "hi".to_string(),
            "AD".to_string(),
            "asd".to_string(),
            "rwer".to_string(),
        ];
        for val in v {
            tx.send(val).unwrap();
            // thread::sleep(Duration::from_millis(1)); //pause for 1 sec between therad executions
        }
        // tx.send(&l).unwrap(); //send the string to main channel, sned method returns Result<T,E>
    });
    for recieved in rx {
        println!("Got: {recieved}");
    }

    /*mutex - mutual exclusion - handles data shared b/w the threads.
    - handles only 1 thread to access the data.
    - a thread has to indicate/signal that it wants the data only then the data will be given to it
    - this handles multiple owners of same data
    - lock is a data str that keeps track of the data taken by a thread
    - call to lock would fail if another holding lock panicks
    - typeof m = Mutex<i32>, lock helps using the inner i32 val
    -
    */
    let m = Mutex::new(7);
    {
        let n = m.lock().unwrap();
        println!("{n:?}");
        /*without unwrap == Ok(7), type of m.lock = core::result::Result<std::sync::poison::mutex::MutexGuard<i32>,std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<i32>>>
        MutexGuard<i32> - this is a smart pointer -
            - has Deref -> to point to inner data
            - has Drop -> Drops the data when going out of scope
        type of n with unwrap = std::sync::poison::mutex::MutexGuard<i32> = also type of m
        */
        print_type_of_var(&n);
    }
    println!("m = {m:?}");
    print_type_of_var(&m);

    /*multiple chnnls-
        moving a var to multiple chnnls at a time is not viable so, Rc<T> is used
        Rc<T> are not safe to be shared across threadds and Arc<T> -> Atomic Reference Count is safe to share across threads
    */
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 1..11 {
        let new_counter= Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut n = new_counter.lock().unwrap();
            *n += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
