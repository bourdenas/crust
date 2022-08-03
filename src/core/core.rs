use super::FpsCounter;
use crate::{
    action::{ActionExecutor, ActionQueue, Index, ACTION_QUEUE, INDEX},
    components::{Animation, Collisions, Id, Position, RigidBody, SpriteInfo, Velocity},
    core::{renderer, EventPump, Status},
    crust::{user_input, Action, UserInput},
    event::EventManager,
    input::InputManager,
    resources::{SpriteManager, TextureManager},
    scene::SceneManager,
    systems::{AnimatorSystem, CollisionSystem, MovementSystem},
};
use sdl2::{
    image::{self, InitFlag},
    render::WindowCanvas,
};
use specs::prelude::*;
use std::{
    sync::mpsc::{self, Sender},
    time::{Duration, SystemTime},
};

pub struct Core {
    resource_path: String,

    pub world: World,
    pub executor: ActionExecutor,
    pub input_manager: InputManager,
    pub event_manager: EventManager,
    pub scene_manager: SceneManager,

    fps_counter: FpsCounter,

    tx: Sender<Action>,

    _sdl_context: sdl2::Sdl,
    _video_subsystem: sdl2::VideoSubsystem,
    _image_context: sdl2::image::Sdl2ImageContext,
    event_pump: EventPump,
    canvas: WindowCanvas,
}

impl Core {
    pub fn init(resource_path: &str, width: u32, height: u32) -> Result<Self, Status> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let event_pump = EventPump::new(&sdl_context)?;

        let window = video_subsystem
            .window("crust demo", width, height)
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
        world.register::<SpriteInfo>();
        world.register::<Velocity>();
        world.register::<Animation>();
        world.register::<Collisions>();
        world.register::<RigidBody>();

        let sprite_manager = SpriteManager::create(resource_path);
        world.insert(sprite_manager);
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
            executor: ActionExecutor::new(rx),
            input_manager: InputManager::new(),
            event_manager: EventManager::new(),
            scene_manager: SceneManager::new(resource_path),
            fps_counter: FpsCounter::new(),
            tx,
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

        let animations = AnimatorSystem::new(ActionQueue::new(self.tx.clone()));
        let movement = MovementSystem::new();
        let collisions = CollisionSystem::new(ActionQueue::new(self.tx.clone()));

        let mut dispatcher = DispatcherBuilder::new()
            .with(animations, "Animation", &[])
            .with(movement, "Movement", &["Animation"])
            .with(collisions, "Collisions", &["Animation", "Movement"])
            .build();
        dispatcher.setup(&mut self.world);

        let mut prev_time = SystemTime::now();

        'game: loop {
            self.fps_counter.start_frame();

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
            self.executor.process(
                &mut self.world,
                &mut self.scene_manager,
                &mut self.event_manager,
            );

            // Update time.
            let curr_time = SystemTime::now();
            let time_since_last_frame = curr_time.duration_since(prev_time).unwrap();
            *self.world.write_resource::<Duration>() = time_since_last_frame;
            self.fps_counter.progress(time_since_last_frame);
            prev_time = curr_time;

            dispatcher.dispatch(&mut self.world);

            // Apply any incoming Actions as a result of systems being dispatched.
            self.executor.process(
                &mut self.world,
                &mut self.scene_manager,
                &mut self.event_manager,
            );

            self.world.maintain();
            self.render(&mut texture_manager);

            self.fps_counter.end_frame();
        }
    }

    pub fn halt(&self) {}

    fn render(&mut self, texture_manager: &mut TextureManager<sdl2::video::WindowContext>) {
        if let Err(e) = renderer::render(
            &mut self.canvas,
            &self.scene_manager,
            texture_manager,
            self.world.system_data(),
        ) {
            println!("{}", e);
        }
    }
}
