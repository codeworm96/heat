use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::sync::Arc;
use std::thread;
use std::time::SystemTime;

use clap::{App, Arg};

mod broker;
mod pid;

use broker::{Broker, Task};

fn parse_config(filename: &str) -> io::Result<Vec<f32>> {
    let config = File::open(filename)?;
    let buffered = BufReader::new(config);

    let mut res = Vec::new();

    for line in buffered.lines() {
        let mut cnt = 0;
        for c in line?.chars() {
            if c != '|' {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "unexpected character!",
                ));
            }
            cnt += 1;
        }
        if cnt > 10 {
            cnt = 10
        } else if cnt == 0 {
            continue;
        }
        res.push(cnt as f32 / 10.0)
    }

    Ok(res)
}

fn main() {
    let matches = App::new("HEAT")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .get_matches();
    let config = matches.value_of("config").unwrap_or("default.conf");

    let cpu_num = num_cpus::get();
    let mut threads = Vec::with_capacity(cpu_num);

    println!("Running. Number of CPUs = {}.", cpu_num);

    let target = parse_config(config).expect("failed to parse config");

    let broker = Arc::new(std::sync::Mutex::new(Broker::new(target)));

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
