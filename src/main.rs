
// modules
mod multiple_threads;

// required methods
use multiple_threads::multithreaded;

fn main() {
    println!("Hello, world {}!", multithreaded(3, 5));
}
