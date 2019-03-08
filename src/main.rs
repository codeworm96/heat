use std::thread;

fn main() {
    let cpu_num = num_cpus::get();
    let mut threads = Vec::with_capacity(cpu_num);
    for _ in 0..cpu_num {
        threads.push(thread::spawn(|| { println!("Hello, world!") }));
    }

    for t in threads {
        t.join().unwrap();
    }
}
