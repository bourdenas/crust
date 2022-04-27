use crate::core::SceneManager;
use crate::sdl;
use crate::sdl::EventPump;
use crate::sdl::TextureManager;
use crate::trust::user_input;
use crate::Status;
use sdl2::image::{self, InitFlag};
use sdl2::render::{Texture, WindowCanvas};
use std::time::{Duration, SystemTime};

pub struct Core {
    _sdl_context: sdl2::Sdl,
    _video_subsystem: sdl2::VideoSubsystem,
    _image_context: sdl2::image::Sdl2ImageContext,
    event_pump: sdl::EventPump,
    canvas: WindowCanvas,
    scene_manager: SceneManager,
}

impl Core {
    pub fn init() -> Result<Self, Status> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let event_pump = EventPump::new(&sdl_context)?;

        let window = video_subsystem
            .window("trust demo", 800, 600)
            .position_centered()
            .build()
            .expect("could not initialize video subsystem");
        let canvas = window
            .into_canvas()
            .build()
            .expect("could not make a canvas");

        Ok(Core {
            _sdl_context: sdl_context,
            _video_subsystem: video_subsystem,
            _image_context: image::init(InitFlag::PNG | InitFlag::JPG)?,
            event_pump,
            canvas,
            scene_manager: SceneManager::new(),
        })
    }

    pub fn run(&mut self) {
        let texture_creator = self.canvas.texture_creator();
        let mut texture_manager = TextureManager::new(&texture_creator);
        let texture = texture_manager
            .load("assets/bardo.png")
            .expect("Failed to load 'assets/reaper.png'");

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
            self.render(&texture);
            self.end_frame(&time_since_last_frame);

            prev_time = curr_time;

            // Try to cap 60fps.
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    pub fn halt(&self) {}

    fn start_frame(&self, _time_since_last_frame: &Duration) {}

    fn end_frame(&self, _time_since_last_frame: &Duration) {}

    fn render(&mut self, texture: &Texture) {
        self.scene_manager.render(&mut self.canvas, texture);
    }
}
