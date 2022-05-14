//! Exam is a library and a cargo plugin to ensure your source code is at a good state by applying a series of inspections.
//!
//! We have the following inspections available, with more to come:
//! - rustfmt
//! - clippy
//! - `cargo test`

use std::error::Error;

use exams::{cargo_test, clippy, rustfmt};

pub fn apply() -> Result<(), Vec<Box<dyn Error>>> {
    let mut failed_exams = vec![];

    macro_rules! apply_exam {
        ($exam:tt) => {
            println!("Applying {}...", stringify!($exam));
            collect_failures(&mut failed_exams, $exam);
        };
    }

    apply_exam!(rustfmt);
    apply_exam!(clippy);
    apply_exam!(cargo_test);

    if failed_exams.is_empty() {
        Ok(())
    } else {
        Err(failed_exams)
    }
}

fn collect_failures<F, T, E>(failed_exams: &mut Vec<Box<dyn Error>>, apply_exam: F)
where
    F: FnOnce() -> Result<T, E>,
    E: Error + 'static,
{
    if let Err(e) = apply_exam() {
        failed_exams.push(Box::new(e));
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ExamError {
    #[error(transparent)]
    RustFmt(#[from] exams::RustFmtError),
}

pub mod exams;
