use std::process::Command;

pub fn cargo_test() -> Result<(), CargoTestError> {
    let output = Command::new("cargo")
        .arg("test")
        .arg("-q")
        .output()
        .expect("cargo should be installed");

    if output.status.success() {
        Ok(())
    } else {
        Err(CargoTestError::TestsFailed)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CargoTestError {
    #[error("Some tests didn't pass")]
    TestsFailed,
}
