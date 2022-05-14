use std::process::Command;

use crate::Exam;

use super::from_output;

pub struct TestsExam;

impl Exam for TestsExam {
    fn name(&self) -> &str {
        "cargo tests"
    }

    fn apply(&mut self) -> Result<(), crate::ExamFailure> {
        let output = Command::new("cargo")
            .arg("test")
            .arg("-q")
            .output()
            .expect("cargo should be installed");

        if output.status.success() {
            Ok(())
        } else {
            Err(from_output(TestsError::TestsFailed, output))
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TestsError {
    #[error("Some tests didn't pass")]
    TestsFailed,
}
