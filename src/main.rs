use std::thread;
use std::time::Duration;

fn main() {
    
    let v = vec![1, 2, 3, 4];

    let handle =  thread::spawn( move || {
        println!("Vector {:?}", v);
    });

    for i in 1..5 {
        println!("i: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
