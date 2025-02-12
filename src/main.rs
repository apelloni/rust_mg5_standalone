use colored::Colorize;
use rand::Rng;
use std::time::{Duration, Instant};

use rust_mg5::uux_aag::RustMG5;
use rust_mg5::{MG5Integrand, MG5Parameters};

/// Benchmarking Function for the estimation of the evaluation time
/// for a given number of evaluation it returns the mean execution time
/// together with the standard deviation
pub fn bench<F, T>(mut f: F, cycles: usize) -> (T, Duration, Duration)
where
    F: FnMut() -> T,
{
    let mut eval_time = vec![];
    let mut start: Instant;
    let mut end: Instant;
    for _ in 0..cycles {
        start = Instant::now();
        f();
        end = Instant::now();
        eval_time.push(end.duration_since(start).as_nanos() as f32);
    }
    let res = f();

    let mean: f32 = eval_time.iter().sum::<f32>() / cycles as f32;
    let variance: f32 = eval_time
        .iter()
        .map(|value| {
            let diff = mean - *value;
            diff * diff
        })
        .sum::<f32>()
        / cycles as f32;

    let t_mean = Duration::from_nanos(mean as u64);
    let t_std = Duration::from_nanos(variance.sqrt() as u64);
    (res, t_mean, t_std)
}

fn main() {
    // Initialize MG5
    let card_path = "./standalone_sm_ma/Cards/param_card.dat";
    let mut mg5_integrand = RustMG5::init(card_path);
    //println!("====\n{}====\n",mg5_integrand.cout());

    // Set Momenta
    let moms: Vec<[f64; 4]> = vec![
        [6.500000e+03, 0.000000e+00, 0.000000e+00, 6.500000e+03],
        [6.500000e+03, 0.000000e+00, 0.000000e+00, -6.500000e+03],
        [5.961499e+03, 2.202625e+03, 4.934899e+03, -2.515227e+03],
        [4.733171e+03, -2.382594e+02, -4.519608e+03, 1.382377e+03],
        [2.305330e+03, -1.964365e+03, -4.152913e+02, 1.132850e+03],
    ];
    let moms_flat: Vec<f64> = moms.into_iter().flatten().collect();

    mg5_integrand.set_externals(&moms_flat);
    let res = mg5_integrand.evaluate();
    println!("res = {res:.5e}");
    println!("ninitial = {}", mg5_integrand.n_initials());
    println!("nexternal = {}", mg5_integrand.n_externals());
    println!("name = {}", mg5_integrand.name().bold().yellow());

    /* ============================================
     * START Benchmark
     * ============================================*/
    let n_samples = 10000;
    let dimensions = 4 * mg5_integrand.n_externals();

    // Set random number generator
    let mut rng = rand::rng();

    // Evaluate multiple random points
    let (res, t_mean, t_std) = bench(
        || {
            let p: Vec<f64> = (0..dimensions).map(|_| rng.random::<f64>()).collect();
            mg5_integrand.set_externals(&p);
            mg5_integrand.evaluate()
        },
        n_samples,
    );

    /* ============================================
     * Print Report
     * ============================================*/
    let time_str = format!("{:?} +/- {:?} ({:E} samples)", t_mean, t_std, n_samples);
    println!("{}: {}", "Evaluation time".bold(), time_str.green());
    print!("{}: ", "Last result".bold());
    println!("{}", res);

    println!("{}", mg5_integrand.cout());

    println!("as = {}", mg5_integrand.aS());
}
