use std::sync::Arc;
use std::thread;
use std::time::SystemTime;

mod broker;
mod pid;

use broker::{Broker, Task};

fn main() {
    let cpu_num = num_cpus::get();
    let mut threads = Vec::with_capacity(cpu_num);

    println!("Running. Number of CPUs = {}.", cpu_num);

    let broker = Arc::new(std::sync::Mutex::new(Broker::new(vec![0.3, 0.7, 0.2, 0.4])));

    for _ in 0..cpu_num {
        let broker = broker.clone();
        threads.push(thread::spawn(move || loop {
            let task = broker.lock().unwrap().next();
            match task {
                Task::Spin(t) => {
                    let now = SystemTime::now();
                    loop {
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
