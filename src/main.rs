use cxx::CxxVector;
use libc::{c_char, c_double, c_int};
use std::{env, ffi::OsString, path::PathBuf, str};

#[cxx::bridge]
mod ffi {

    unsafe extern "C++" {
        include!("md5_class.h");

        type MD5Integrand;

        fn new_md5_integrand() -> UniquePtr<MD5Integrand>;
        //fn init(self: Pin<&mut MD5Integrand>);
        //fn init(self: Pin<&mut MD5Integrand>, );
        unsafe fn init(self: Pin<&mut MD5Integrand>, card_path: *const c_char);
        unsafe fn set_momenta(self: Pin<&mut MD5Integrand>, data: *const f64, size: usize);
        fn get_matrix_element(self: Pin<&mut MD5Integrand>) -> f64;

        // Consts used for internal array limits
        fn ninitial(&self) -> usize;
        fn nexternal(&self) -> usize;
        fn nprocesses(&self) -> usize;

    }
}

//unsafe impl cxx::ExternType for ffi::MD5Integrand {
//    type Id = cxx::type_id!("md5::MD5Integrand");
//    type Kind = cxx::kind::Opaque;
//}

fn main() {
    let card_path = "./standalone_uubar_aag/Cards/param_card.dat";
    let mut md5_integrand = ffi::new_md5_integrand();

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
    //(*md5_integrand).init();
}
