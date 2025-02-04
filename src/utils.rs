/// Wrapper for rust of the CPP bridge and the function calls
macro_rules! rmg5 {
    (
        $mg5_bridge:ty
    ) => {
        /// Wrapper for rust of the CPP bridge and the function calls
        pub struct RustMG5 {
            integrand: cxx::UniquePtr<$mg5_bridge>,
            initialized: bool,
        }

        impl Default for RustMG5 {
            fn default() -> Self {
                let mg5_integrand = ffi::new_mg5_integrand();
                // At the moment only a single process is supported
                assert!(mg5_integrand.nprocesses() == 1);
                Self {
                    integrand: mg5_integrand,
                    initialized: false,
                }
            }
        }
        impl crate::MG5Integrand for RustMG5 {
            /// Initialise integrand with the parameter card
            fn init(card_path: &str) -> Self {
                let mut mg5_integrand = ffi::new_mg5_integrand();
                // At the moment only a single process is supported
                assert!(mg5_integrand.nprocesses() == 1);
                // Import Parameter Card
                assert!(
                    std::path::Path::new(card_path).exists(),
                    "Path {} doesn't exist!",
                    card_path
                );
                unsafe {
                    let c_path = std::ffi::CString::new(card_path).unwrap();
                    mg5_integrand.as_mut().unwrap().init(c_path.as_ptr());
                }
                Self {
                    integrand: mg5_integrand,
                    initialized: true,
                }
            }

            /// Set integrand with the parameter card
            fn set_card(&mut self, card_path: &str) {
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
            fn set_externals(&mut self, flatten_moms: &[f64]) {
                // Flatten data
                unsafe {
                    self.integrand
                        .as_mut()
                        .unwrap()
                        .set_momenta(flatten_moms.as_ptr(), flatten_moms.len())
                }
            }

            /// Evaluate matrix element
            fn evaluate(&mut self) -> f64 {
                assert!(self.initialized, "Set Parameter Card before evaluating");
                self.integrand.as_mut().unwrap().get_matrix_element()
            }

            /// Get number of external momenta
            fn n_externals(&self) -> usize {
                self.integrand.nexternal()
            }
            /// Get number of initial momenta
            fn n_initials(&self) -> usize {
                self.integrand.ninitial()
            }
            /// Get number of processes
            fn n_processes(&self) -> usize {
                self.integrand.nprocesses()
            }
            /// Get process name
            fn name(&self) -> String {
                self.integrand.get_name().to_string()
            }

            /// Obtain CPP stdout string
            fn cout(&mut self) -> String {
                self.integrand.as_mut().unwrap().read_cout().to_string()
            }
        }
    };
}
