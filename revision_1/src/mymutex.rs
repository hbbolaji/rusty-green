use std::{ops::AddAssign, sync::Mutex, thread, time::Duration};

pub fn test_mutext() {
  let mut score = Mutex::new(0u16);
  // let unlocked_data = score.lock();
  // let mut data = unlocked_data.unwrap();
  // data.add_assign(5);
  // println!("{:?}", data);

  let myfunc_1 = || {
    loop {
      thread::sleep(Duration::from_millis(200));
      println!("Thread 1 is waiting for Mutex loc...");
      let guard = score.try_lock();
      if guard.is_ok() {
        let mut data = guard.unwrap();
        for i in 1..10 {
          data.add_assign(i);
          println!("Thread 1 is adding {i} = {}", data);
        }
        break;
      }
      thread::sleep(Duration::from_millis(50));    
    }
  };

  let myfunc_2 = || {
    println!("Thread 2 is waiting for Mutex loc...");
    let mut data = score.lock().unwrap();
    for i in 1..10 {
      data.add_assign(i);
      println!("Thread 2 is adding {i} = {}", data);
      thread::sleep(Duration::from_millis(300));
    }
  };

  // _ = thread::spawn(myfunc).join();
  thread::scope(|scope| {
    scope.spawn(myfunc_1);
    scope.spawn(myfunc_2);
  });

  println!("{:?}", score.lock().unwrap())
}
