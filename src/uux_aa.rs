/// Bridge to the standalone CPP library from MadGraph5
/// It allows to set the momenenta and evaluate the matrix_element
///
/// Process u u~ > a a
///
#[cxx::bridge(namespace = "MG5_sm_ma_uux_aa")]
pub mod ffi {

    unsafe extern "C++" {
        //include!("cpp/mg5_sm_ma_uux_aag_ddx.h");
        include!("cpp/mg5_sm_ma_uux_aa.h");

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
    }
}

rmg5!(ffi::MG5Integrand);
