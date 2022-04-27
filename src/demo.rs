use trust::core::Core;
use trust::Status;

fn main() -> Result<(), Status> {
    let mut core = Core::init()?;
    core.run();
    Ok(())
}
