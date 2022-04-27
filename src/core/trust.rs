use crate::core::SceneManager;
use crate::sdl;
use crate::trust::user_input;
use crate::Status;
use std::time::{Duration, SystemTime};

pub struct Trust {
    sdl: sdl::Sdl,
    event_pump: sdl::EventPump,
    scene_manager: SceneManager,
}

impl Trust {
    pub fn init() -> Result<Self, Status> {
        let sdl = sdl::Sdl::init()?;
        let event_pump = sdl.create_event_pump()?;
        let scene_manager = SceneManager::new(sdl.create_canvas("trust demo", 800, 600));

        Ok(Trust {
            sdl,
            event_pump,
            scene_manager,
        })
    }

    pub fn run(&mut self) {
        let mut prev_time = SystemTime::now();

        'game: loop {
            // Input event handling
            'events: loop {
                match self.event_pump.poll().event {
                    Some(user_input::Event::NoEvent(..)) => {
                        break 'events;
                    }
                    Some(user_input::Event::QuitEvent(..)) => {
                        break 'game;
                    }
                    Some(user_input::Event::KeyEvent(event)) if event.key == "Q" => break 'game,
                    Some(user_input::Event::KeyEvent(event)) => println!("key: {:#?}", event),
                    Some(event) => println!("{:#?}", event),
                    _ => {}
                }
            }

            let curr_time = SystemTime::now();
            let time_since_last_frame = curr_time.duration_since(prev_time).unwrap();

            self.start_frame(&time_since_last_frame);
            self.render();
            self.end_frame(&time_since_last_frame);

            prev_time = curr_time;

            // Try to cap 60fps.
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    pub fn halt(&self) {}

    fn start_frame(&self, _time_since_last_frame: &Duration) {}

    fn end_frame(&self, _time_since_last_frame: &Duration) {}

    fn render(&mut self) {
        self.scene_manager.render();
    }
}
