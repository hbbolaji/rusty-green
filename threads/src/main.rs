use std::{sync::Mutex, thread, time::Duration};

fn main() {
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("Hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..5 {
    //     println!("Hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // handle.join().unwrap();
    let v: Mutex<Vec<i32>> = Mutex::new(vec![1, 2, 3]);
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // let handle2 = thread::spawn(move || {
    //     println!("Here is another ref to the vector: {:?}", v1);
    // });

    handle.join().unwrap();
    // handle2.join().unwrap();
}
