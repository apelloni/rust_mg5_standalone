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
                // Lock function for the internal initialization of Prameters
                let _lock = crate::GLOBAL_LOCK.lock().unwrap();

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
                // Lock function for the internal initialization of Prameters
                let _lock = crate::GLOBAL_LOCK.lock().unwrap();

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

        /// Model Parameters
        impl crate::MG5Parameters for RustMG5 {
            // Obtain model parameters independent of aS
            fn mdl_WH(&self) -> f64 {
                self.integrand.get_mdl_WH()
            }
            fn mdl_WW(&self) -> f64 {
                self.integrand.get_mdl_WW()
            }
            fn mdl_WZ(&self) -> f64 {
                self.integrand.get_mdl_WZ()
            }
            fn mdl_WT(&self) -> f64 {
                self.integrand.get_mdl_WT()
            }
            fn mdl_ymtau(&self) -> f64 {
                self.integrand.get_mdl_ymtau()
            }
            fn mdl_ymt(&self) -> f64 {
                self.integrand.get_mdl_ymt()
            }
            fn mdl_ymb(&self) -> f64 {
                self.integrand.get_mdl_ymb()
            }
            fn aS(&self) -> f64 {
                self.integrand.get_aS()
            }
            fn mdl_Gf(&self) -> f64 {
                self.integrand.get_mdl_Gf()
            }
            fn aEWM1(&self) -> f64 {
                self.integrand.get_aEWM1()
            }
            fn mdl_MH(&self) -> f64 {
                self.integrand.get_mdl_MH()
            }
            fn mdl_MZ(&self) -> f64 {
                self.integrand.get_mdl_MZ()
            }
            fn mdl_MTA(&self) -> f64 {
                self.integrand.get_mdl_MTA()
            }
            fn mdl_MT(&self) -> f64 {
                self.integrand.get_mdl_MT()
            }
            fn mdl_MB(&self) -> f64 {
                self.integrand.get_mdl_MB()
            }
            fn mdl_CKM3x3(&self) -> f64 {
                self.integrand.get_mdl_CKM3x3()
            }
            fn mdl_conjg_CKM1x1(&self) -> f64 {
                self.integrand.get_mdl_conjg_CKM1x1()
            }
            fn mdl_conjg_CKM3x3(&self) -> f64 {
                self.integrand.get_mdl_conjg_CKM3x3()
            }
            fn mdl_MZ_exp_2(&self) -> f64 {
                self.integrand.get_mdl_MZ_exp_2()
            }
            fn mdl_MZ_exp_4(&self) -> f64 {
                self.integrand.get_mdl_MZ_exp_4()
            }
            fn mdl_sqrt_2(&self) -> f64 {
                self.integrand.get_mdl_sqrt_2()
            }
            fn mdl_MH_exp_2(&self) -> f64 {
                self.integrand.get_mdl_MH_exp_2()
            }
            fn mdl_aEW(&self) -> f64 {
                self.integrand.get_mdl_aEW()
            }
            fn mdl_MW(&self) -> f64 {
                self.integrand.get_mdl_MW()
            }
            fn mdl_sqrt_aEW(&self) -> f64 {
                self.integrand.get_mdl_sqrt_aEW()
            }
            fn mdl_ee(&self) -> f64 {
                self.integrand.get_mdl_ee()
            }
            fn mdl_MW_exp_2(&self) -> f64 {
                self.integrand.get_mdl_MW_exp_2()
            }
            fn mdl_sw2(&self) -> f64 {
                self.integrand.get_mdl_sw2()
            }
            fn mdl_cw(&self) -> f64 {
                self.integrand.get_mdl_cw()
            }
            fn mdl_sqrt_sw2(&self) -> f64 {
                self.integrand.get_mdl_sqrt_sw2()
            }
            fn mdl_sw(&self) -> f64 {
                self.integrand.get_mdl_sw()
            }
            fn mdl_g1(&self) -> f64 {
                self.integrand.get_mdl_g1()
            }
            fn mdl_gw(&self) -> f64 {
                self.integrand.get_mdl_gw()
            }
            fn mdl_vev(&self) -> f64 {
                self.integrand.get_mdl_vev()
            }
            fn mdl_vev_exp_2(&self) -> f64 {
                self.integrand.get_mdl_vev_exp_2()
            }
            fn mdl_lam(&self) -> f64 {
                self.integrand.get_mdl_lam()
            }
            fn mdl_yb(&self) -> f64 {
                self.integrand.get_mdl_yb()
            }
            fn mdl_yt(&self) -> f64 {
                self.integrand.get_mdl_yt()
            }
            fn mdl_ytau(&self) -> f64 {
                self.integrand.get_mdl_ytau()
            }
            fn mdl_muH(&self) -> f64 {
                self.integrand.get_mdl_muH()
            }
            fn mdl_ee_exp_2(&self) -> f64 {
                self.integrand.get_mdl_ee_exp_2()
            }
            fn mdl_sw_exp_2(&self) -> f64 {
                self.integrand.get_mdl_sw_exp_2()
            }
            fn mdl_cw_exp_2(&self) -> f64 {
                self.integrand.get_mdl_cw_exp_2()
            }
            fn mdl_complexi(&mut self) -> num::Complex<f64> {
                let v = self.integrand.as_mut().unwrap().get_mdl_complexi();
                num::Complex::new(*v.get(0).unwrap(), *v.get(1).unwrap())
            }
            fn mdl_I1x33(&mut self) -> num::Complex<f64> {
                let v = self.integrand.as_mut().unwrap().get_mdl_I1x33();
                num::Complex::new(*v.get(0).unwrap(), *v.get(1).unwrap())
            }
            fn mdl_I2x33(&mut self) -> num::Complex<f64> {
                let v = self.integrand.as_mut().unwrap().get_mdl_I2x33();
                num::Complex::new(*v.get(0).unwrap(), *v.get(1).unwrap())
            }
            fn mdl_I3x33(&mut self) -> num::Complex<f64> {
                let v = self.integrand.as_mut().unwrap().get_mdl_I3x33();
                num::Complex::new(*v.get(0).unwrap(), *v.get(1).unwrap())
            }
            fn mdl_I4x33(&mut self) -> num::Complex<f64> {
                let v = self.integrand.as_mut().unwrap().get_mdl_I4x33();
                num::Complex::new(*v.get(0).unwrap(), *v.get(1).unwrap())
            }

            // Model parameters dependent on aS
            fn mdl_sqrt_aS(&self) -> f64 {
                self.integrand.get_mdl_sqrt_aS()
            }
            fn G(&self) -> f64 {
                self.integrand.get_G()
            }
            fn mdl_G_exp_2(&self) -> f64 {
                self.integrand.get_mdl_G_exp_2()
            }

            // Model couplings independent of aS
            fn GC_2(&mut self) -> num::Complex<f64> {
                let v = self.integrand.as_mut().unwrap().get_GC_2();
                num::Complex::new(*v.get(0).unwrap(), *v.get(1).unwrap())
            }

            // Model couplings dependent on aS
            fn GC_11(&mut self) -> num::Complex<f64> {
                let v = self.integrand.as_mut().unwrap().get_GC_11();
                num::Complex::new(*v.get(0).unwrap(), *v.get(1).unwrap())
            }
        }
    };
}
