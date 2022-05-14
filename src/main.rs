fn main() {
    println!("Exam time started!\n");
    let result = exam::apply();
    println!();

    if let Err(failed_exams) = result {
        println!("Unfortunately, your repository didn't go well on the exams.");
        println!("Here's what it could learn to perform better in the future:\n");

        failed_exams.iter().for_each(|failed| println!("{failed}"));
        std::process::exit(1);
    } else {
        println!("Passed on all exams!");
    }
}
