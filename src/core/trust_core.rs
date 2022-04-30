use crate::components::{Animated, Position, Sprite};
use crate::core::{renderer, EventPump, Status, TextureManager};
use crate::resources::SpriteSheetsManager;
use crate::systems::Keyboard;
use crate::trust::{user_input, Animation, AnimationScript, FrameRangeAnimation, KeyEvent};
use sdl2::image::{self, InitFlag};
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use specs::prelude::*;
use std::time::{Duration, SystemTime};

pub struct Core {
    _sdl_context: sdl2::Sdl,
    _video_subsystem: sdl2::VideoSubsystem,
    _image_context: sdl2::image::Sdl2ImageContext,
    event_pump: EventPump,
    canvas: WindowCanvas,
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
        })
    }

    pub fn run(&mut self) {
        let texture_creator = self.canvas.texture_creator();
        let mut texture_manager = TextureManager::new(&texture_creator);

        let mut world = World::new();
        world.register::<Position>();
        world.register::<Sprite>();
        world.register::<Animated>();

        let mut dispatcher = DispatcherBuilder::new()
            .with(Keyboard, "Keyboard", &[])
            .build();
        dispatcher.setup(&mut world);

        let key_events: Vec<KeyEvent> = vec![];
        world.insert(key_events);

        let sheets_manager = SpriteSheetsManager::new();
        world.insert(sheets_manager);

        world
            .create_entity()
            .with(Position(Point::new(400, 300)))
            .with(Sprite {
                resource: "reaper".to_owned(),
                frame_index: 3,
            })
            .with(Animated {
                script: AnimationScript {
                    id: "walk_down".to_owned(),
                    animation: vec![Animation {
                        id: "walk".to_owned(),
                        wait_all: false,
                        frame_range: Some(FrameRangeAnimation {
                            start_frame: 0,
                            end_frame: 3,
                            delay: 200,
                            repeat: 0,
                            ..Default::default()
                        }),
                        ..Default::default()
                    }],
                    repeat: 0,
                },
            })
            .build();

        let mut prev_time = SystemTime::now();
        world.insert(Duration::ZERO);

        'game: loop {
            // Input event handling.
            let mut key_events = vec![];
            'events: loop {
                match self.event_pump.poll().event {
                    Some(user_input::Event::NoEvent(..)) => {
                        break 'events;
                    }
                    Some(user_input::Event::QuitEvent(..)) => {
                        break 'game;
                    }
                    Some(user_input::Event::KeyEvent(event)) if event.key == "Q" => break 'game,
                    Some(user_input::Event::KeyEvent(event)) => key_events.push(event),
                    // Some(event) => println!("{:#?}", event),
                    _ => {}
                }
            }
            *world.write_resource::<Vec<KeyEvent>>() = key_events;

            // Update time.
            let curr_time = SystemTime::now();
            let time_since_last_frame = curr_time.duration_since(prev_time).unwrap();
            *world.write_resource::<Duration>() = time_since_last_frame;

            // self.start_frame(&time_since_last_frame);
            dispatcher.dispatch(&mut world);
            world.maintain();

            self.render(&mut texture_manager, &world);

            // self.end_frame(&time_since_last_frame);
            prev_time = curr_time;

            // Try to cap 60fps.
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    pub fn halt(&self) {}

    // fn start_frame(&self, _time_since_last_frame: &Duration) {}

    // fn end_frame(&self, _time_since_last_frame: &Duration) {}

    fn render(
        &mut self,
        texture_manager: &mut TextureManager<sdl2::video::WindowContext>,
        world: &World,
    ) {
        if let Err(e) = renderer::render(&mut self.canvas, texture_manager, world.system_data()) {
            println!("{}", e);
        }
    }
}
