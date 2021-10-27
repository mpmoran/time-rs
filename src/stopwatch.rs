use std::time::Duration;

pub struct Stopwatch {
    length: Duration,
    is_running: bool,
}

impl Stopwatch {
    pub fn new() -> Stopwatch {
        Stopwatch {
            length: Duration::from_secs(0),
            is_running: false,
        }
    }

    // pub fn as_string(&self) -> String {
    //     format!("{}", self.as_u64())
    // }

    pub fn get_length_as_f64(&self) -> f64 {
        self.get_length_as_uint() as f64 / 60.0
    }

    pub fn get_length_as_uint(&self) -> u64 {
        self.length.as_secs()
    }

    pub fn advance(&mut self, num_seconds: u64) {
        self.length += Duration::from_secs(num_seconds);
    }

    pub fn start(&mut self) {
        self.is_running = true;
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }

    pub fn reset(&mut self) {
        self.is_running = false;
        self.length = Duration::from_secs(0);
    }
}
