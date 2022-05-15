//! Exam is a library and a cargo plugin to ensure your source code is at a good state by applying a series of inspections.
//!
//! We have the following inspections available, with more to come:
//! - rustfmt
//! - clippy
//! - `cargo test`

use std::{
    any::{type_name, TypeId},
    collections::HashMap,
    error::Error,
};

use downcast_rs::{impl_downcast, Downcast};
use exams::*;

pub trait Exam: Downcast {
    fn name(&self) -> &str {
        type_name::<Self>()
    }

    fn apply(&mut self) -> Result<(), ExamFailure>;
}

impl_downcast!(Exam);

pub struct ExamFailure {
    pub error: Box<dyn Error + 'static>,
    pub report: Option<String>,
}

pub struct Examiner {
    map: HashMap<TypeId, Box<dyn Exam + 'static>>,
}

impl std::fmt::Debug for Examiner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Examiner")
            .field(
                "exams",
                &self
                    .map
                    .iter()
                    .map(|(k, v)| (k, v.name()))
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

impl Examiner {
    pub fn new() -> Self {
        let mut this = Self::empty();

        this.add(RustfmtExam).add(TestsExam).add(ClippyExam);

        this
    }

    pub fn empty() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn add<T: Exam + 'static>(&mut self, exam: T) -> &mut Self {
        let exam: Box<dyn Exam> = Box::new(exam);
        self.map.insert(TypeId::of::<T>(), exam);
        self
    }

    pub fn delete<T: Exam + 'static>(&mut self) -> &mut Self {
        self.remove::<T>();
        self
    }

    pub fn remove<T: Exam + 'static>(&mut self) -> Option<T> {
        self.map.remove(&TypeId::of::<T>()).map(|exam| {
            *exam
                .downcast::<T>()
                .map_err(|_| panic!("This downcast shouldn't fail"))
                .unwrap()
        })
    }

    pub fn apply(&mut self) -> Result<(), Vec<ExamFailure>> {
        let mut failed_exams = vec![];

        let mut run = |exam| {
            apply_collecting_failures(&mut failed_exams, exam);
        };

        for exam in self.map.values_mut() {
            println!("Applying {}...", exam.name());
            run(exam);
        }

        if failed_exams.is_empty() {
            Ok(())
        } else {
            Err(failed_exams)
        }
    }
}

impl Default for Examiner {
    fn default() -> Self {
        Self::new()
    }
}

fn apply_collecting_failures(failed_exams: &mut Vec<ExamFailure>, exam: &mut Box<dyn Exam>) {
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
