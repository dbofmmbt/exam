use std::process::Command;

use crate::Exam;

use super::{from_io_err, from_output};

pub struct ClippyExam;

impl Exam for ClippyExam {
    fn name(&self) -> &str {
        "clippy"
    }

    fn apply(&mut self) -> Result<(), crate::ExamFailure> {
        let output = Command::new("cargo")
            .arg("clippy")
            .arg("--")
            .arg("-D")
            .arg("warnings")
            .output()
            .map_err(from_io_err)?;

        if output.status.success() {
            Ok(())
        } else {
            Err(from_output(ClippyError::LintViolation, output))
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ClippyError {
    #[error("clippy command wasn't found. Perhaps you could install it? {0}")]
    NotInstalled(#[from] std::io::Error),
    #[error("Lint violations were found")]
    LintViolation,
}
