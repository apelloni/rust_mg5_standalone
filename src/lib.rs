use std::time::{Duration, Instant};

use ffi::MG5Integrand;

/// Wrapper for rust of the CPP bridge and the function calls
pub struct RustMG5 {
    integrand: cxx::UniquePtr<MG5Integrand>,
    initialized: bool,
    p_data: Vec<f64>, // Container for the momenta parsing
}

impl Default for RustMG5 {
    fn default() -> Self {
        let mg5_integrand = ffi::new_mg5_integrand();
        // At the moment only a single process is supported
        assert!(mg5_integrand.nprocesses() == 1);
        Self {
            integrand: mg5_integrand,
            initialized: false,
            p_data: vec![],
        }
    }
}

impl RustMG5 {
    /// Initialise integrand with the parameter card
    pub fn set_card(&mut self, card_path: &str) {
        // Import Parameter Card
        assert!(
            std::path::Path::new(card_path).exists(),
            "Path {} doesn't exist!",
            card_path
        );
        unsafe {
            let c_path = std::ffi::CString::new(card_path).unwrap();
            self.integrand.as_mut().unwrap().init(c_path.as_ptr());
        }
        self.initialized = true;
    }

    /// Set momenta for the evaluation
    pub fn set_externals(&mut self, flatten_moms: &[f64]) {
        // Flatten data
        unsafe {
            self.integrand
                .as_mut()
                .unwrap()
                .set_momenta(flatten_moms.as_ptr(), flatten_moms.len())
        }
    }

    /// Evaluate matrix element
    pub fn evaluate(&mut self) -> f64 {
        assert!(self.initialized,"Set Parameter Card before evaluating");
        self.integrand.as_mut().unwrap().get_matrix_element()
    }

    /// Get number of external momenta
    pub fn n_externals(&self) -> usize {
        self.integrand.nexternal()
    }
    /// Get number of initial momenta
    pub fn n_initials(&self) -> usize {
        self.integrand.ninitial()
    }
    /// Get number of processes
    pub fn n_processes(&self) -> usize {
        self.integrand.nprocesses()
    }
    /// Get process name
    pub fn name(&self) -> String {
        self.integrand.get_name().to_string()
    }
}

/// Bridge to the standalone CPP library from MadGraph5
/// It allows to set the momenenta and evaluate the matrix_element
#[cxx::bridge]
mod ffi {

    unsafe extern "C++" {
        include!("mg5_class.h");

        type MG5Integrand;

        fn new_mg5_integrand() -> UniquePtr<MG5Integrand>;

        // Initialize with parameter_card
        unsafe fn init(self: Pin<&mut MG5Integrand>, card_path: *const c_char);

        // Compute x-section
        unsafe fn set_momenta(self: Pin<&mut MG5Integrand>, data: *const f64, size: usize);
        fn get_matrix_element(self: Pin<&mut MG5Integrand>) -> f64;

        // Consts used for internal array limits
        fn ninitial(&self) -> usize;
        fn nexternal(&self) -> usize;
        fn nprocesses(&self) -> usize;
        fn get_name(&self) -> &CxxString;

    }
}
