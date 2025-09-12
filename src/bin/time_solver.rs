use std::time::Instant;
use wave_function_collapse::constraint_solver::SolverState;

fn main() {
    let mut solver = SolverState::new();
    let start = Instant::now();
    let solve_result = solver.solve();
    let duration = start.elapsed();
    println!("Solve time: {:?} ({})", duration, if solve_result {"success"} else {"failed"});
}