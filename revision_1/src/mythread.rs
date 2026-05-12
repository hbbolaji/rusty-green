use std::{thread, time::Instant};

pub fn test_threads() {
    let mut x = 0u128;

    for i in 1..50_000_000 {
        x += i;
    }
    println!("The value of x in main thread is {x}")
}

pub fn test_spawn_thread() {
    println!("Starting new worker thread");
    let now = Instant::now();
    let handle_1 = thread::spawn(|| {
        let mut x = 0u128;

        for i in 1..500_000_000 {
            x += i;
        }
        println!("The value of x in thread 1 is {x}")
    });

    let handle_2 = thread::spawn(|| {
        let mut x = 0u128;

        for i in 1..500_000_000 {
            x += i;
        }
        println!("The value of x in thread 2 is {x}")
    });

    // handle_1.join();
    // handle_2.join();

    loop {
        test_threads();
        if handle_1.is_finished() && handle_2.is_finished() {
            println!("All workers are done, let's get out of here");
            break;
        }
    }

    println!(
        "Worker threads completed... in {:?} seconds",
        now.elapsed().as_secs()
    )
}
