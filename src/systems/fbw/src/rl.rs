#[derive(Debug)]
pub(crate) struct RateLimiter {
    y_slope: f64,
    in_: [f64; 2],
    out: [f64; 2],
}

impl Default for RateLimiter {
    fn default() -> Self {
        RateLimiter {
            y_slope: 0.0,
            in_: [0.0, 0.0],
            out: [0.0, 0.0],
        }
    }
}

impl RateLimiter {
    pub(crate) fn reset(&mut self, i: f64) {
        self.in_ = [i, i];
        self.out = [i, i];
    }

    pub(crate) fn iterate(&mut self, i: f64, rate: f64, dt: std::time::Duration) -> f64 {
        let dt = dt.as_secs_f64();

        self.in_[0] = i;

        let slope = (self.in_[0] - self.out[1]) / dt;

        if slope <= -rate {
            self.y_slope = -rate;
        }

        if -rate <= slope && slope <= rate {
            self.y_slope = slope;
        }

        if rate <= slope {
            self.y_slope = rate;
        }

        self.out[0] = self.out[1] + (dt * self.y_slope);
        self.out[1] = self.out[0];
        self.in_[1] = self.in_[0];

        self.out[0]
    }
}