///! This module implements a simple PID controller with moving average.

const MAX_NUM_DATA : usize = 1;

pub struct PidController {
    // PID-Parameters
    p_coeff : f32,
    i_coeff : f32,
    d_coeff : f32,

    // Total number of data points
    num_data : usize,

    // Collected datapoints, stored in circular buffer
    data_points : [f32; MAX_NUM_DATA],

    // Accumulated error over time
    last_avg : f32,
    acc_avg : f32,
}

impl PidController {
    fn average(&self) -> f32 {
        let max_idx = if self.num_data < MAX_NUM_DATA { self.num_data } else { MAX_NUM_DATA };
        let mut sum : f32 = 0.0;
        for i in 0..max_idx {
            sum += self.data_points[i];
        };
        sum / (max_idx as f32)
    }

    pub fn new(p : f32, i : f32, d : f32) -> Self {
        PidController {
            p_coeff : p,
            i_coeff : i,
            d_coeff : d,
            num_data : 0,
            data_points : [0.0; MAX_NUM_DATA],
            last_avg : 0.0,
            acc_avg : 0.0,
        }
    }

    pub fn next_output(&mut self, new_sample : f32) -> f32 {
        // Put current data into buffer
        let curr_idx = self.num_data % MAX_NUM_DATA;
        self.data_points[curr_idx] = new_sample;

        // Retrieve current moving everage
        self.num_data += 1;
        let curr_avg = self.average();
        
        // Retrieve different
        let diff = curr_avg - self.last_avg;
        self.last_avg = curr_avg;

        // Update I-term
        self.acc_avg += curr_avg; // Update I-term
        
        // Out = E * p + (\Int E dt) * i + (\Diff E dt) * d
        curr_avg * self.p_coeff + self.acc_avg * self.i_coeff + diff * self.d_coeff
    }
}


#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_pcoeff() {
        let mut c1 = super::PidController::new(-1.0, 0.0, 0.0);
        assert_approx_eq!(-1.0, c1.next_output(1.0), 1e-2f32);
        assert_approx_eq!(-2.0, c1.next_output(2.0), 1e-2f32);
        assert_approx_eq!(-2.0, c1.next_output(2.0), 1e-2f32);
        assert_approx_eq!(4.0, c1.next_output(-4.0), 1e-2f32);
    }

    #[test]
    fn test_icoeff() {
        let mut c1 = super::PidController::new(0.0, -1.0, 0.0);
        assert_approx_eq!(-1.0, c1.next_output(1.0), 1e-2f32);
        assert_approx_eq!(-3.0, c1.next_output(2.0), 1e-2f32);
        assert_approx_eq!(-5.0, c1.next_output(2.0), 1e-2f32);
        assert_approx_eq!(-1.0, c1.next_output(-4.0), 1e-2f32);
    }
}
