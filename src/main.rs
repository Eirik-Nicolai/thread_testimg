use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    
    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone();

    thread::spawn( move || {
        let vals = [
            String::from("hi"),
            String::from("from"),
            String::from("beyond"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });

    thread::spawn( move || {
        let vals = [
            String::from("hi"),
            String::from("again"),
            String::from("hello"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });

    for recvd in rx {
        println!("Received {}", recvd);
    }
}
