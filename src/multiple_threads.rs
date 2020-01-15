
use std::thread;
use std::time::Duration;

pub fn multithreaded(x : i32, y : i32) -> i32 {
    let res = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(100 * i));
        }
    });

    for i in 1..6 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(50 * i));
    }

    let result = res.join();

    x + y
}
