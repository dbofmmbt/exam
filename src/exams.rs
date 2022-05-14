mod rustfmt;
use std::process::Output;

use crate::ExamFailure;

pub use self::rustfmt::{RustFmtError, RustfmtExam};

mod clippy;
pub use self::clippy::{ClippyError, ClippyExam};

mod tests;
pub use self::tests::{TestsError, TestsExam};

fn from_io_err(error: std::io::Error) -> ExamFailure {
    ExamFailure {
        error: error.into(),
        report: None,
    }
}

fn from_output<E: std::error::Error + 'static>(error: E, output: Output) -> ExamFailure {
    ExamFailure {
        error: error.into(),
        report: Some(String::from_utf8_lossy(&output.stdout).into()),
    }
}
