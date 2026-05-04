use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    let handle = thread::spawn(move || {
        // let message = String::from("Hello!");
        // tx.send(message).unwrap()
        let vals = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("new"),
            String::from("thread")
        ];

        for v in vals {
            tx.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let handle2 = thread::spawn(move || {
        let vals = vec![
            String::from("More"),
            String::from("messages"),
            String::from("from"),
            String::from("another"),
            String::from("thread")
        ];

        for v in vals {
            tx2.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);
    for received in rx {
        println!("Got: {}", received);
    }
    handle.join().unwrap();
    handle2.join().unwrap();
}
