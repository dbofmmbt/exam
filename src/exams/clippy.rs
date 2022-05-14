use std::process::Command;

pub fn clippy() -> Result<(), ClippyError> {
    let output = Command::new("cargo")
        .arg("clippy")
        .arg("--")
        .arg("-D")
        .arg("warnings")
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(ClippyError::LintViolation)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ClippyError {
    #[error("clippy command wasn't found. Perhaps you could install it? {0}")]
    NotInstalled(#[from] std::io::Error),
    #[error("Lint violations were found")]
    LintViolation,
}
