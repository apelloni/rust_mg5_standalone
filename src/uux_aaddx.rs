/// Bridge to the standalone CPP library from MadGraph5
/// It allows to set the momenenta and evaluate the matrix_element
///
/// Process u u~ > a a g ( g > d d~)
///
#[cxx::bridge(namespace = "MG5_sm_ma_uux_aaddx")]
pub mod ffi {

    unsafe extern "C++" {
        //include!("cpp/mg5_sm_ma_uux_aag_ddx.h");
        include!("cpp/mg5_sm_ma_uux_aaddx.h");

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

        // Obtain CPP cout string
        fn read_cout(self: Pin<&mut MG5Integrand>) -> &CxxString;

        // Obtain model parameters independent of aS
        fn get_mdl_WH(&self) -> f64;
        fn get_mdl_WW(&self) -> f64;
        fn get_mdl_WZ(&self) -> f64;
        fn get_mdl_WT(&self) -> f64;
        fn get_mdl_ymtau(&self) -> f64;
        fn get_mdl_ymt(&self) -> f64;
        fn get_mdl_ymb(&self) -> f64;
        fn get_aS(&self) -> f64;
        fn get_mdl_Gf(&self) -> f64;
        fn get_aEWM1(&self) -> f64;
        fn get_mdl_MH(&self) -> f64;
        fn get_mdl_MZ(&self) -> f64;
        fn get_mdl_MTA(&self) -> f64;
        fn get_mdl_MT(&self) -> f64;
        fn get_mdl_MB(&self) -> f64;
        fn get_mdl_CKM3x3(&self) -> f64;
        fn get_mdl_conjg_CKM1x1(&self) -> f64;
        fn get_mdl_conjg_CKM3x3(&self) -> f64;
        fn get_mdl_MZ_exp_2(&self) -> f64;
        fn get_mdl_MZ_exp_4(&self) -> f64;
        fn get_mdl_sqrt_2(&self) -> f64;
        fn get_mdl_MH_exp_2(&self) -> f64;
        fn get_mdl_aEW(&self) -> f64;
        fn get_mdl_MW(&self) -> f64;
        fn get_mdl_sqrt_aEW(&self) -> f64;
        fn get_mdl_ee(&self) -> f64;
        fn get_mdl_MW_exp_2(&self) -> f64;
        fn get_mdl_sw2(&self) -> f64;
        fn get_mdl_cw(&self) -> f64;
        fn get_mdl_sqrt_sw2(&self) -> f64;
        fn get_mdl_sw(&self) -> f64;
        fn get_mdl_g1(&self) -> f64;
        fn get_mdl_gw(&self) -> f64;
        fn get_mdl_vev(&self) -> f64;
        fn get_mdl_vev_exp_2(&self) -> f64;
        fn get_mdl_lam(&self) -> f64;
        fn get_mdl_yb(&self) -> f64;
        fn get_mdl_yt(&self) -> f64;
        fn get_mdl_ytau(&self) -> f64;
        fn get_mdl_muH(&self) -> f64;
        fn get_mdl_ee_exp_2(&self) -> f64;
        fn get_mdl_sw_exp_2(&self) -> f64;
        fn get_mdl_cw_exp_2(&self) -> f64;
        fn get_mdl_complexi(self: Pin<&mut MG5Integrand>) -> &CxxVector<f64>;
        fn get_mdl_I1x33(self: Pin<&mut MG5Integrand>) -> &CxxVector<f64>;
        fn get_mdl_I2x33(self: Pin<&mut MG5Integrand>) -> &CxxVector<f64>;
        fn get_mdl_I3x33(self: Pin<&mut MG5Integrand>) -> &CxxVector<f64>;
        fn get_mdl_I4x33(self: Pin<&mut MG5Integrand>) -> &CxxVector<f64>;

        // Model parameters dependent on aS
        fn get_mdl_sqrt_aS(&self) -> f64;
        fn get_G(&self) -> f64;
        fn get_mdl_G_exp_2(&self) -> f64;

        // Model couplings independent of aS
        fn get_GC_2(self: Pin<&mut MG5Integrand>) -> &CxxVector<f64>;

        // Model couplings dependent on aS
        fn get_GC_11(self: Pin<&mut MG5Integrand>) -> &CxxVector<f64>;

    }
}

rmg5!(ffi::MG5Integrand);
