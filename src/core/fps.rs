use std::time::{Duration, SystemTime};

pub struct FpsCounter {
    pub fps: u32,
    frame_count: u32,
    elapsed_time: Duration,

    frame_started: SystemTime,
    best_frame_duration: Duration,
    worst_frame_duration: Duration,
}

impl Default for FpsCounter {
    fn default() -> Self {
        Self {
            fps: Default::default(),
            frame_count: Default::default(),
            elapsed_time: Default::default(),
            frame_started: SystemTime::now(),
            best_frame_duration: Default::default(),
            worst_frame_duration: Default::default(),
        }
    }
}

impl FpsCounter {
    pub fn new() -> Self {
        FpsCounter::default()
    }

    pub fn start_frame(&mut self) {
        self.frame_started = SystemTime::now();
    }
    pub fn end_frame(&mut self) {
        self.frame_count += 1;

        let frame_duration = SystemTime::now()
            .duration_since(self.frame_started)
            .unwrap();

        if self.best_frame_duration == Duration::ZERO || frame_duration < self.best_frame_duration {
            self.best_frame_duration = frame_duration;
        }
        if self.worst_frame_duration < frame_duration {
            self.worst_frame_duration = frame_duration;
        }

        if self.elapsed_time >= Duration::from_millis(1000) {
            self.fps = self.frame_count;
            self.frame_count = 0;
            self.elapsed_time -= Duration::from_millis(1000);

            println!(
                "🦀 fps: {} (avg: {:.2} msec, best: {:.2} msec, worst: {:.2} msec",
                self.fps,
                1000.0 / self.fps as f32,
                self.best_frame_duration.as_millis(),
                self.worst_frame_duration.as_millis(),
            );

            self.best_frame_duration = Duration::ZERO;
            self.worst_frame_duration = Duration::ZERO;
        }
    }

    pub fn progress(&mut self, time_since_last_frame: Duration) {
        self.elapsed_time += time_since_last_frame;
    }
}
