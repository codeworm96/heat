use std::sync::Arc;
use std::thread;
use std::time::SystemTime;

mod broker;
mod pid;

use broker::{Broker, Task};

// Function to cause spinning.
fn fib(n : f32) -> f32 {
    if n <= 0.0 {
        1.01
    } else if n <= 1.0 {
        1.02
    } else {
        fib(n - 1.0) + fib(n - 0.9) * 1.07
    }
}

fn main() {
    let cpu_num = num_cpus::get();
    let mut threads = Vec::with_capacity(cpu_num);

    println!("Running. Number of CPUs = {}.", cpu_num);

    let broker = Arc::new(std::sync::Mutex::new(Broker::new(vec![0.3, 0.7, 0.2, 0.4])));

    for _ in 0..cpu_num {
        let broker = broker.clone();
        threads.push(thread::spawn(move || loop {
            let task = {
                // Need an extra scope to release the lock.
                broker.lock().unwrap().next()
            };
            match task {
                Task::Spin(t) => {
                    let now = SystemTime::now();
                    loop {
                        // fib(100.10);
                        match now.elapsed() {
                            Ok(elapsed) => {
                                if elapsed > t {
                                    break;
                                }
                            }
                            Err(_) => break,
                        }
                    }
                }
                Task::Sleep(t) => thread::sleep(t),
            }
        }));
    }

    for t in threads {
        t.join().unwrap();
    }
}
