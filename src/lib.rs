use runner::{print_results_table, run_solvers};
use solver::Solver;

mod runner;
pub mod solver;

pub fn create_results_table<T: Solver, F: Fn(&T) -> bool>(
    label_headers: &[&str],
    solvers: &[T],
    solver_predicate: F,
) {
    let (row_formats, max_length) = run_solvers(label_headers, solvers, solver_predicate);
    print_results_table(label_headers, &row_formats, &max_length);
}
