use crate::action::ActionExecutor;
use crate::components::{Position, ScriptState, Sprite};
use crate::core::{renderer, EventPump, Status, TextureManager};
use crate::resources::SpriteSheetsManager;
use crate::systems::{Keyboard, ScriptSystem};
use crate::trust::{user_input, KeyEvent};
use sdl2::image::{self, InitFlag};
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use specs::prelude::*;
use std::time::{Duration, SystemTime};

pub struct Core {
    resource_path: String,

    pub world: World,
    pub executor: ActionExecutor,

    _sdl_context: sdl2::Sdl,
    _video_subsystem: sdl2::VideoSubsystem,
    _image_context: sdl2::image::Sdl2ImageContext,
    event_pump: EventPump,
    canvas: WindowCanvas,
}

impl Core {
    pub fn init(resource_path: &str) -> Result<Self, Status> {
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

        let mut world = World::new();
        world.register::<Position>();
        world.register::<Sprite>();
        world.register::<ScriptState>();

        Ok(Core {
            resource_path: resource_path.to_owned(),
            world,
            executor: ActionExecutor::new(),
            _sdl_context: sdl_context,
            _video_subsystem: video_subsystem,
            _image_context: image::init(InitFlag::PNG | InitFlag::JPG)?,
            event_pump,
            canvas,
        })
    }

    pub fn run(&mut self) {
        let texture_creator = self.canvas.texture_creator();
        let mut texture_manager = TextureManager::new(&self.resource_path, &texture_creator);

        let mut dispatcher = DispatcherBuilder::new()
            .with(Keyboard, "Keyboard", &[])
            .with(ScriptSystem::default(), "Scripts", &[])
            .build();
        dispatcher.setup(&mut self.world);

        self.world.insert(Duration::ZERO);

        let key_events: Vec<KeyEvent> = vec![];
        self.world.insert(key_events);

        let sheets_manager = SpriteSheetsManager::new();
        self.world.insert(sheets_manager);

        self.world
            .create_entity()
            .with(Position(Point::new(400, 300)))
            .with(Sprite {
                resource: "reaper".to_owned(),
                frame_index: 3,
            })
            // .with(FrameRangeState::new(FrameRangeAnimation {
            //     start_frame: 0,
            //     end_frame: 3,
            //     delay: 200,
            //     // repeat: 3,
            //     // horizontal_align: HorizontalAlign::Right as i32,
            //     ..Default::default()
            // }))
            // .with(TranslationState::new(VectorAnimation {
            //     vec: Some(Vector {
            //         x: 0.0,
            //         y: 1.0,
            //         z: 0.0,
            //     }),
            //     delay: 16,
            //     ..Default::default()
            // }))
            .build();

        let mut prev_time = SystemTime::now();

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
            *self.world.write_resource::<Vec<KeyEvent>>() = key_events;

            // Update time.
            let curr_time = SystemTime::now();
            let time_since_last_frame = curr_time.duration_since(prev_time).unwrap();
            *self.world.write_resource::<Duration>() = time_since_last_frame;

            // self.start_frame(&time_since_last_frame);
            dispatcher.dispatch(&mut self.world);
            self.world.maintain();

            self.render(&mut texture_manager);

            // self.end_frame(&time_since_last_frame);
            prev_time = curr_time;
        }
    }

    pub fn halt(&self) {}

    // fn start_frame(&self, _time_since_last_frame: &Duration) {}

    // fn end_frame(&self, _time_since_last_frame: &Duration) {}

    fn render(&mut self, texture_manager: &mut TextureManager<sdl2::video::WindowContext>) {
        if let Err(e) =
            renderer::render(&mut self.canvas, texture_manager, self.world.system_data())
        {
            println!("{}", e);
        }
    }
}
