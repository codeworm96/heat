use std::sync::Arc;
use std::thread;
use std::time::SystemTime;

mod broker;
mod pid;

use broker::{Broker, Task};

fn main() {
    let cpu_num = num_cpus::get();
    let mut threads = Vec::with_capacity(cpu_num);
    // let mut controller = pid::PidController::new();

    // println!("Controller says {}.", controller.next_output(1.0));
    // println!("Controller says {}.", controller.next_output(1.0));
    // println!("Controller says {}.", controller.next_output(1.0));
    // println!("Controller says {}.", controller.next_output(1.0));

    let broker = Arc::new(spin::Mutex::new(Broker::new(vec![0.3, 0.7])));

    for _ in 0..cpu_num {
        let broker = broker.clone();
        threads.push(thread::spawn(move || loop {
            match broker.lock().next() {
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
