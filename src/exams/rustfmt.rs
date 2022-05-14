use std::process::Command;

pub fn rustfmt() -> Result<(), RustFmtError> {
    let output = Command::new("cargo")
        .arg("fmt")
        .arg("--")
        .arg("--check")
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        // We don't really need to handle it, as it is a best-effort attempt.
        let _ = Command::new("cargo").arg("fmt").spawn();
        Err(RustFmtError::Unformatted)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RustFmtError {
    #[error("rustfmt command wasn't found. Perhaps you could install it? {0}")]
    NotInstalled(#[from] std::io::Error),
    #[error("unformatted code was found")]
    Unformatted,
}
