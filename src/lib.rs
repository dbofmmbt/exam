//! Exam is a library and a cargo plugin to ensure your source code is at a good state by applying a series of inspections.
//!
//! We have the following inspections available, with more to come:
//! - rustfmt
//! - clippy
//! - `cargo test`

use std::error::Error;

use exams::*;

pub trait Exam {
    fn apply(&mut self) -> Result<(), ExamFailure>;
}

impl<F> Exam for F
where
    F: FnMut() -> Result<(), ExamFailure>,
{
    fn apply(&mut self) -> Result<(), ExamFailure> {
        self()
    }
}

pub struct ExamFailure {
    pub error: Box<dyn Error + 'static>,
    pub report: Option<String>,
}

pub fn apply() -> Result<(), Vec<ExamFailure>> {
    let mut failed_exams = vec![];

    macro_rules! apply_exam {
        ($exam:ident) => {
            println!("Applying {}...", stringify!($exam));
            collect_failures(&mut failed_exams, $exam);
        };
    }

    apply_exam!(RustfmtExam);
    apply_exam!(ClippyExam);
    apply_exam!(TestsExam);

    if failed_exams.is_empty() {
        Ok(())
    } else {
        Err(failed_exams)
    }
}

fn collect_failures(failed_exams: &mut Vec<ExamFailure>, mut exam: impl Exam) {
    if let Err(e) = exam.apply() {
        failed_exams.push(e);
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ExamError {
    #[error(transparent)]
    RustFmt(#[from] exams::RustFmtError),
}

pub mod exams;
