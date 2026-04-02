use criterion::Criterion;
use fun_stuff::mach::thread_policy::force_p_cores;

mod copy_vs_heap;

fn main() {
    force_p_cores();
    println!("Forced P-cores for consistent benchmarking.");

    copy_vs_heap::benches();

    Criterion::default()
        .configure_from_args()
        .final_summary();
}