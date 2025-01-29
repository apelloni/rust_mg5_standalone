use colored::Colorize;
use rand::Rng;
use std::time::{Duration, Instant};

/// Bridge to the standalone CPP library from MadGraph5
/// It allows to set the momenenta and evaluate the matrix_element
#[cxx::bridge]
mod ffi {

    unsafe extern "C++" {
        include!("md5_class.h");

        type MD5Integrand;

        fn new_md5_integrand() -> UniquePtr<MD5Integrand>;

        // Initialize with parameter_card
        unsafe fn init(self: Pin<&mut MD5Integrand>, card_path: *const c_char);

        // Compute x-section
        unsafe fn set_momenta(self: Pin<&mut MD5Integrand>, data: *const f64, size: usize);
        fn get_matrix_element(self: Pin<&mut MD5Integrand>) -> f64;

        // Consts used for internal array limits
        fn ninitial(&self) -> usize;
        fn nexternal(&self) -> usize;
        fn nprocesses(&self) -> usize;
        fn get_name(&self) -> &CxxString;

    }
}

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
    //let card_path = "./standalone_uubar_aag/Cards/param_card.dat";
    let card_path = "./standalone_uubar_aag_ddbar/Cards/param_card.dat";
    let mut md5_integrand = ffi::new_md5_integrand();
    assert!(md5_integrand.nprocesses() == 1);

    // Import Parameter Card
    unsafe {
        let c_path = std::ffi::CString::new(card_path).unwrap();
        md5_integrand.as_mut().unwrap().init(c_path.as_ptr());
    }

    // Set Momenta
    let moms: Vec<[f64; 4]> = vec![
        [6.500000e+03, 0.000000e+00, 0.000000e+00, 6.500000e+03],
        [6.500000e+03, 0.000000e+00, 0.000000e+00, -6.500000e+03],
        [5.961499e+03, 2.202625e+03, 4.934899e+03, -2.515227e+03],
        [4.733171e+03, -2.382594e+02, -4.519608e+03, 1.382377e+03],
        [2.305330e+03, -1.964365e+03, -4.152913e+02, 1.132850e+03],
    ];
    // Flatten data
    let p_data: Vec<f64> = moms.into_iter().flatten().collect();
    unsafe {
        md5_integrand
            .as_mut()
            .unwrap()
            .set_momenta(p_data.as_ptr(), p_data.len())
    }
    let res = md5_integrand.as_mut().unwrap().get_matrix_element();
    println!("res = {res:.5e}");
    println!("ninitial = {}", md5_integrand.ninitial());
    println!("nexternal = {}", md5_integrand.nexternal());
    println!(
        "name = {}",
        md5_integrand.get_name().to_str().unwrap().bold().yellow()
    );

    /* ============================================
     * START Benchmark
     * ============================================*/
    let n_samples = 100000;
    let dimensions = 4 * md5_integrand.nexternal();

    // Set random number generator
    let mut rng = rand::rng();

    // Evaluate multiple random points
    let (res, t_mean, t_std) = bench(
        || {
            let p: Vec<f64> = (0..dimensions).map(|_| rng.random::<f64>()).collect();
            unsafe {
                md5_integrand
                    .as_mut()
                    .unwrap()
                    .set_momenta(p.as_ptr(), p.len())
            }
            md5_integrand.as_mut().unwrap().get_matrix_element()
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
}
