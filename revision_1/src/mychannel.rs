use std::{sync::mpsc, thread, time::Duration};

pub fn test_channels() {
    let (tx, rx) = mpsc::channel::<i32>();
    // let send_result = tx.send(23);
    // tx.send(123);
    // tx.send(12323);

    // if send_result.is_ok() {
    //   println!("Message successfully sent!...");
    // }

    // let rx_result = rx.recv_timeout(Duration::from_millis(300));
    // println!("Result is Okay: {}", rx_result.is_ok());
    // println!("Result is {}", rx_result.unwrap());

    // let rx_result = rx.recv_timeout(Duration::from_millis(300));
    // println!("Result is Okay: {}", rx_result.is_ok());
    // println!("Result is {}", rx_result.unwrap());

    let processor_code = move || {
        println!("Starting process thread...");
        let mut attempt_count = 0u8;
        loop {
            println!(
                "# {} Attempting to recieve message from channel...",
                attempt_count + 1
            );
            let rx_result = rx.recv_timeout(Duration::from_millis(800));
            if rx_result.is_ok() {
                println!("Recieved message {:?}", rx_result.unwrap());
                attempt_count = 0;
            } else {
                if attempt_count < 10 {
                    attempt_count += 1
                } else {
                    println!("Aborting processor thread!..");
                    break;
                }
            }
        }
    };

    for x in 1..6 {
        let tx_result = tx.send(x);
        println!("Sending Status: {}", tx_result.is_ok());
        thread::sleep(Duration::from_millis(400));
    }

    thread::spawn(processor_code).join();
}
