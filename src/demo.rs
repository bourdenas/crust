use trust::core::{Core, Status};

fn main() -> Result<(), Status> {
    let mut core = Core::init()?;
    core.run();
    Ok(())
}
