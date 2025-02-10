/// Wrapper for rust of the CPP bridge and the function calls
macro_rules! rmg5 {
    (
        $mg5_bridge:ty
    ) => {
        /// Wrapper for rust of the CPP bridge and the function calls
        pub struct RustMG5 {
            integrand: cxx::UniquePtr<$mg5_bridge>,
            param_card: Option<String>,
            initialized: bool,
        }

        impl Clone for RustMG5 {
            fn clone(&self) -> Self {
                let mut new_ptr = ffi::new_mg5_integrand();
                if self.initialized {
                    unsafe {
                        let card_path = self.param_card.clone().unwrap();
                        let c_path = std::ffi::CString::new(card_path.as_str()).unwrap();
                        new_ptr.as_mut().unwrap().init(c_path.as_ptr());
                    }
                }
                Self {
                    integrand: new_ptr,
                    param_card: self.param_card.clone(),
                    initialized: self.initialized.clone(),
                }
            }
        }

        impl Default for RustMG5 {
            fn default() -> Self {
                let mg5_integrand = ffi::new_mg5_integrand();
                // At the moment only a single process is supported
                assert!(mg5_integrand.nprocesses() == 1);
                Self {
                    integrand: mg5_integrand,
                    param_card: None,
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
                    param_card: Some(String::from(card_path)),
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
                self.param_card = Some(String::from(card_path));
                self.initialized = true;
            }

            /// Set momenta for the evaluation
            fn set_externals(&mut self, flatten_moms: &[f64]) {
                assert_eq!(
                    self.n_externals() * 4,
                    flatten_moms.len(),
                    "Expected vector of length {} but found {}",
                    self.n_externals() * 4,
                    flatten_moms.len()
                );
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
