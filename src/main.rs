use libc::c_char;
use std::{env, ffi::OsString, path::PathBuf, str};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("md5_class.h");

        type MD5Integrand;

        fn new_md5_integrand() -> UniquePtr<MD5Integrand>;
        //fn init(self: Pin<&mut MD5Integrand>);
        fn init(self: Pin<&mut MD5Integrand>, );
    }
}

//unsafe impl cxx::ExternType for ffi::MD5Integrand {
//    type Id = cxx::type_id!("md5::MD5Integrand");
//    type Kind = cxx::kind::Opaque;
//}

fn main() {
    let mut md5_integrand = ffi::new_md5_integrand();
    //let c_path = std::ffi::CString::new("../").unwrap();
    md5_integrand.as_mut().unwrap().init();
    //(*md5_integrand).init();
}
