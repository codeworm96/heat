use super::pid;
use rand::prelude::*;
///！ This module is the job broker. It dispatches job according to current system load
/// and expected load.
///
use std::time;
use systemstat::{data, Platform, System};

// How many ticks count as one "bar"
const TIME_SCALE: u128 = 10000u128;
const CPU_LOAD_SAMPLE: i64 = 300;
const DEFAULT_TASK: time::Duration = time::Duration::from_millis(1);

pub enum Task {
    Spin(time::Duration),
    Sleep(time::Duration),
}

pub struct Broker {
    pid: pid::PidController,
    target: Vec<f32>,
    last_target: f32,
    instant: time::Instant,
    // Load object to record current cpu load
    load_object: data::DelayedMeasurement<data::CPULoad>,
    last_cpu_load: (f32, i64),
}

fn get_load_object() -> data::DelayedMeasurement<data::CPULoad> {
    let sys = System::new();
    match sys.cpu_load_aggregate() {
        Ok(cpu) => cpu,
        Err(e) => panic!("Error gettign cpu object: {}.\n", e),
    }
}

impl Broker {
    pub fn new(target: Vec<f32>) -> Self {
        let last_target = target[0];
        Broker {
            pid: pid::PidController::new(-0.0001, -0.0001, 0.0),
            target: target,
            last_target: last_target,
            instant: time::Instant::now(),
            load_object: get_load_object(),
            last_cpu_load: (0.0, -2 * CPU_LOAD_SAMPLE),
        }
    }

    pub fn next(&mut self) -> Task {
        let time = self.instant.elapsed().as_millis();

        print!("Time = {}. ", time);

        // Current CPU Utilization
        let (last_sample, last_timestamp) = self.last_cpu_load;
        let curr_util = if time as i64 - last_timestamp > CPU_LOAD_SAMPLE {
            let load_stat = self.load_object.done().unwrap();
            let overall_load = 1.0 - load_stat.idle;
            self.load_object = get_load_object();
            self.last_cpu_load = (overall_load, time as i64);
            print!("New system load = {}. ", overall_load);
            overall_load
        } else {
            print!("Last system load = {}. ", last_sample);
            last_sample
        };

        // We use the last target value to compute the error to
        // avoid a suden jump when the target changed
        let error = curr_util - self.last_target;
        let pidval = self.pid.next_output(error);

        // Find current target utilization based on current time

        let bar_idx = ((time / TIME_SCALE) as usize) % self.target.len();
        let target = self.target[bar_idx];
        self.last_target = target;

        print!("Target util = {}. Pid Val = {}. ", target, pidval);

        // Clamp the random threshold.
        let rand_threshold = match target + pidval {
            c if c > 0.9 => 0.9,
            c if c < 0.05 => 0.05,
            c => c,
        };

        // By default it should be a value within (0, 1)
        let randval: f32 = random();

        if randval < rand_threshold {
            println!(" SPIN.");
            Task::Spin(DEFAULT_TASK)
        } else {
            println!(" SLEEP.");
            Task::Sleep(DEFAULT_TASK)
        }
    }
}
