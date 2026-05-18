#![allow(unused_variables)]

use std::{thread, time::Duration};
use futures::{future::FutureExt, join, pin_mut, select};

pub fn test_async() {
  let num_1 = get_number_1().fuse();
  let num_2 = get_number_2().fuse();
  let num_3 = get_number_3().fuse();

  pin_mut!(num_1, num_2, num_3);

  let result = smol::block_on(async {
    // join!(num_1, num_2, num_3)
    loop {
      select! {
        x = num_1 => println!("Num 1 is completed {}", x),
        x = num_2 => println!("Num 2 is completed {}", x),
        x = num_3 => println!("Num 3 is completed {}", x),
        complete => {
          println!("All futures finished polling");
          break;
        }
      };
    }
  }); // Executor

  println!("Final value is {:?}", result);
}

async fn get_number_1() -> u8 {
  // println!("Running function");
  0
}

async fn get_number_2() -> u8 {
  thread::sleep(Duration::from_millis(50));
  50
}

async fn get_number_3() -> u8 {
  thread::sleep(Duration::from_millis(75));
  75
}