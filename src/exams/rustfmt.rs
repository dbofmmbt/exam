use std::process::Command;

use crate::Exam;

use super::{from_io_err, from_output};

pub struct RustfmtExam;

impl Exam for RustfmtExam {
    fn apply(&mut self) -> Result<(), crate::ExamFailure> {
        let output = Command::new("cargo")
            .arg("fmt")
            .arg("--")
            .arg("--check")
            .output()
            .map_err(from_io_err)?;

        if output.status.success() {
            Ok(())
        } else {
            // We don't really need to handle it, as it is a best-effort attempt.
            let _ = Command::new("cargo").arg("fmt").spawn();
            Err(from_output(RustFmtError::Unformatted, output))
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RustFmtError {
    #[error("rustfmt command wasn't found. Perhaps you could install it? {0}")]
    NotInstalled(#[from] std::io::Error),
    #[error("unformatted code was found")]
    Unformatted,
}
