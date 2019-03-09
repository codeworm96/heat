use std::thread;
mod pid;
mod broker;

fn main() {
    let cpu_num = num_cpus::get();
    let mut threads = Vec::with_capacity(cpu_num);
    // let mut controller = pid::PidController::new();

    // println!("Controller says {}.", controller.next_output(1.0));
    // println!("Controller says {}.", controller.next_output(1.0));
    // println!("Controller says {}.", controller.next_output(1.0));
    // println!("Controller says {}.", controller.next_output(1.0));

    for _ in 0..cpu_num {
        threads.push(thread::spawn(|| { println!("Hello, world!") }));
    }

    for t in threads {
        t.join().unwrap();
    }
}
