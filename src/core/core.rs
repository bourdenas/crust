use crate::action::{ActionExecutor, ActionQueue, Index, ACTION_QUEUE, INDEX};
use crate::components::{Id, Position, ScriptState, Sprite};
use crate::core::{renderer, EventPump, Status, TextureManager};
use crate::crust::{user_input, Action, UserInput};
use crate::event::EventManager;
use crate::input::InputManager;
use crate::resources::SpriteSheetsManager;
use crate::systems::{CollisionSystem, ScriptSystem};
use sdl2::image::{self, InitFlag};
use sdl2::render::WindowCanvas;
use specs::prelude::*;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::{Duration, SystemTime};

pub struct Core {
    resource_path: String,

    pub world: World,
    pub executor: ActionExecutor,
    pub input_manager: InputManager,
    pub event_manager: EventManager,

    tx: Sender<Action>,
    rx: Receiver<Action>,

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
        world.register::<Id>();
        world.register::<Position>();
        world.register::<Sprite>();
        world.register::<ScriptState>();

        let sheets_manager = SpriteSheetsManager::new(resource_path);
        world.insert(sheets_manager);
        world.insert(Duration::ZERO);

        let (tx, rx) = mpsc::channel();
        ACTION_QUEUE.with(|queue| {
            *queue.borrow_mut() = Some(ActionQueue::new(tx.clone()));
        });
        INDEX.with(|index| {
            *index.borrow_mut() = Some(Index::new());
        });

        Ok(Core {
            resource_path: resource_path.to_owned(),
            world,
            executor: ActionExecutor::new(),
            input_manager: InputManager::new(),
            event_manager: EventManager::new(),
            tx,
            rx,
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
            .with(ScriptSystem::new(self.tx.clone()), "Scripts", &[])
            .with(
                CollisionSystem::new(self.tx.clone()),
                "Collisions",
                &["Scripts"],
            )
            .build();
        dispatcher.setup(&mut self.world);

        let mut prev_time = SystemTime::now();

        'game: loop {
            // Input event handling.
            'events: loop {
                match self.event_pump.poll().event {
                    Some(user_input::Event::NoEvent(..)) => {
                        break 'events;
                    }
                    Some(user_input::Event::QuitEvent(..)) => {
                        break 'game;
                    }
                    Some(user_input::Event::KeyEvent(event)) if event.key == "Q" => break 'game,
                    Some(user_input::Event::KeyEvent(event)) => {
                        self.input_manager.handle(UserInput {
                            event: Some(user_input::Event::KeyEvent(event)),
                        });
                    }
                    // Some(event) => println!("{:#?}", event),
                    _ => {}
                }
            }

            // Apply any incoming Actions as a result of input handling.
            self.rx.try_iter().for_each(|action| {
                self.executor
                    .execute(action, &mut self.world, &mut self.event_manager);
            });

            // Update time.
            let curr_time = SystemTime::now();
            let time_since_last_frame = curr_time.duration_since(prev_time).unwrap();
            *self.world.write_resource::<Duration>() = time_since_last_frame;

            dispatcher.dispatch(&mut self.world);

            // Apply any incoming Actions as a result of systems being dispatched.
            self.rx.try_iter().for_each(|action| {
                self.executor
                    .execute(action, &mut self.world, &mut self.event_manager);
            });

            self.world.maintain();
            self.render(&mut texture_manager);

            prev_time = curr_time;
        }
    }

    pub fn halt(&self) {}

    fn render(&mut self, texture_manager: &mut TextureManager<sdl2::video::WindowContext>) {
        if let Err(e) =
            renderer::render(&mut self.canvas, texture_manager, self.world.system_data())
        {
            println!("{}", e);
        }
    }
}
