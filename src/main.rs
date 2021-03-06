use std::path::Path;

fn main() {
    if !Path::new("Cargo.toml").exists() {
        eprintln!("The exams can only be applied at the root of a Cargo project");
        std::process::exit(1);
    }

    println!("Exam time started!\n");
    let result = exam::Examiner::default().apply();
    println!();

    if let Err(failed_exams) = result {
        println!("Unfortunately, your repository didn't pass on all the exams.");
        println!("Here's what it could learn to perform better in the future:\n");

        failed_exams
            .iter()
            .enumerate()
            .for_each(|(idx, failed)| println!("{}) {}", idx + 1, failed.error));

        std::process::exit(1);
    } else {
        println!("Passed on all exams!");
    }
}
