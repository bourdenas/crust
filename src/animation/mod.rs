mod animated;
mod animator;
mod frame_range;
mod performer;
mod script_runner;
mod testing;
mod timer;
mod translation;

pub use animated::Animated;
pub use frame_range::FrameRangePerformer;
pub use script_runner::ScriptRunner;
pub use timer::TimerPerformer;
pub use translation::TranslationPerformer;

use animator::Animator;
use performer::Performer;
