pub struct Timer {
    pub timer: f32,
    pub duration: f32,
}

impl Timer {
    pub fn new(duration: f32) -> Timer {
        Timer {
            timer: 0.0,
            duration,
        }
    }

    pub fn update(&mut self, delta: f32) {
        if self.timer <= 0.0 {
            self.timer = 0.0;
            return;
        }
        self.timer -= delta;
    }

    pub fn start_override(&mut self, duration: f32) {
        self.timer = duration;
    }

    pub fn start(&mut self) {
        self.timer = self.duration;
    }

    pub fn is_running(&self) -> bool {
        self.timer > 0.0
    }
}