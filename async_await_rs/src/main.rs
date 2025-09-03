use std::{sync::mpsc, thread, time::Duration};
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
    handle.join().unwrap(); //.join blocks 1 thread until the other thread(repersented by handle) finishes executing, here, in this line, the main thread until all the handle threads finish executing and then it runs
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
        let v = vec!["hi".to_string(), "AD".to_string() , "asd".to_string(), "rwer".to_string()];
        for val in v {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1)); //pause for 1 sec between therad executions
        }
        // tx.send(&l).unwrap(); //send the string to main channel, sned method returns Result<T,E>
    });
    for recieved in rx {
        println!("Got: {recieved}");
    }
    
    
}
