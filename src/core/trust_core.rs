use crate::components::{Position, Sprite};
use crate::sdl::{self, EventPump, TextureManager};
use crate::trust::user_input;
use crate::Status;
use sdl2::image::{self, InitFlag};
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use specs::prelude::*;
use std::time::{Duration, SystemTime};

pub struct Core {
    _sdl_context: sdl2::Sdl,
    _video_subsystem: sdl2::VideoSubsystem,
    _image_context: sdl2::image::Sdl2ImageContext,
    event_pump: sdl::EventPump,
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

        let mut dispatcher = DispatcherBuilder::new().build();
        dispatcher.setup(&mut world);

        world
            .create_entity()
            .with(Position(Point::new(400, 300)))
            .with(Sprite {
                resource: String::from("assets/reaper.png"),
                bounding_box: Rect::new(6, 7, 24, 28),
            })
            .build();

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

            // self.start_frame(&time_since_last_frame);
            dispatcher.dispatch(&mut world);
            world.maintain();

            // self.render(&texture);
            if let Err(e) = render(&mut self.canvas, &mut texture_manager, world.system_data()) {
                println!("{}", e);
            }

            // self.end_frame(&time_since_last_frame);
            prev_time = curr_time;

            // Try to cap 60fps.
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    pub fn halt(&self) {}

    fn start_frame(&self, _time_since_last_frame: &Duration) {}

    fn end_frame(&self, _time_since_last_frame: &Duration) {}

    fn render(&mut self, texture: &Texture) {}
}

type SystemData<'a> = (ReadStorage<'a, Position>, ReadStorage<'a, Sprite>);

pub fn render(
    canvas: &mut WindowCanvas,
    texture_manager: &mut TextureManager<sdl2::video::WindowContext>,
    (pos, sprite): SystemData,
) -> Result<(), Status> {
    // canvas.set_draw_color(background);
    canvas.clear();

    for (pos, sprite) in (&pos, &sprite).join() {
        let texture = texture_manager.load(&sprite.resource)?;
        canvas.copy(
            &texture,
            sprite.bounding_box,
            Rect::new(
                pos.0.x(),
                pos.0.y(),
                2 * sprite.bounding_box.width(),
                2 * sprite.bounding_box.height(),
            ),
        )?;
    }

    canvas.present();

    Ok(())
}
