mod sequential_relaxer;
mod relaxation_utils;
// mod relaxation_problem;

fn main() {
    println!("Starting rusty relaxation.");

    if sequential_relaxer::relax() {
        println!("Managed to call sequential relaxer!");
    }
}
