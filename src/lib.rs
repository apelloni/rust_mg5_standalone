#[macro_use]
mod utils;

pub mod uux_aa;
pub mod uux_aaddx;
pub mod uux_aag;

use num::Complex;

pub trait MG5Integrand
where
    Self: Default,
    Self: Clone,
{
    fn init(card_path: &str) -> Self;
    fn set_card(&mut self, card_path: &str);
    fn set_externals(&mut self, flatten_moms: &[f64]);
    fn evaluate(&mut self) -> f64;
    fn n_externals(&self) -> usize;
    fn n_initials(&self) -> usize;
    fn n_processes(&self) -> usize;
    fn name(&self) -> String;
    fn cout(&mut self) -> String;
}

#[allow(non_snake_case)]
pub trait MG5Parameters
where
    Self: Default,
    Self: Clone,
{
    // Model parameters independent of aS
    fn mdl_WH(&self) -> f64;
    fn mdl_WW(&self) -> f64;
    fn mdl_WZ(&self) -> f64;
    fn mdl_WT(&self) -> f64;
    fn mdl_ymtau(&self) -> f64;
    fn mdl_ymt(&self) -> f64;
    fn mdl_ymb(&self) -> f64;
    fn aS(&self) -> f64;
    fn mdl_Gf(&self) -> f64;
    fn aEWM1(&self) -> f64;
    fn mdl_MH(&self) -> f64;
    fn mdl_MZ(&self) -> f64;
    fn mdl_MTA(&self) -> f64;
    fn mdl_MT(&self) -> f64;
    fn mdl_MB(&self) -> f64;
    fn mdl_CKM3x3(&self) -> f64;
    fn mdl_conjg_CKM1x1(&self) -> f64;
    fn mdl_conjg_CKM3x3(&self) -> f64;
    fn mdl_MZ_exp_2(&self) -> f64;
    fn mdl_MZ_exp_4(&self) -> f64;
    fn mdl_sqrt_2(&self) -> f64;
    fn mdl_MH_exp_2(&self) -> f64;
    fn mdl_aEW(&self) -> f64;
    fn mdl_MW(&self) -> f64;
    fn mdl_sqrt_aEW(&self) -> f64;
    fn mdl_ee(&self) -> f64;
    fn mdl_MW_exp_2(&self) -> f64;
    fn mdl_sw2(&self) -> f64;
    fn mdl_cw(&self) -> f64;
    fn mdl_sqrt_sw2(&self) -> f64;
    fn mdl_sw(&self) -> f64;
    fn mdl_g1(&self) -> f64;
    fn mdl_gw(&self) -> f64;
    fn mdl_vev(&self) -> f64;
    fn mdl_vev_exp_2(&self) -> f64;
    fn mdl_lam(&self) -> f64;
    fn mdl_yb(&self) -> f64;
    fn mdl_yt(&self) -> f64;
    fn mdl_ytau(&self) -> f64;
    fn mdl_muH(&self) -> f64;
    fn mdl_ee_exp_2(&self) -> f64;
    fn mdl_sw_exp_2(&self) -> f64;
    fn mdl_cw_exp_2(&self) -> f64;
    fn mdl_complexi(&mut self) -> Complex<f64>;
    fn mdl_I1x33(&mut self) -> Complex<f64>;
    fn mdl_I2x33(&mut self) -> Complex<f64>;
    fn mdl_I3x33(&mut self) -> Complex<f64>;
    fn mdl_I4x33(&mut self) -> Complex<f64>;
    // Model parameters dependent on aS
    fn mdl_sqrt_aS(&self) -> f64;
    fn G(&self) -> f64;
    fn mdl_G_exp_2(&self) -> f64;
    // Model couplings independent of aS
    fn GC_2(&mut self) -> Complex<f64>;
    // Model couplings dependent on aS
    fn GC_11(&mut self) -> Complex<f64>;
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn clone() {
        let integrand = uux_aag::RustMG5::init("./mg5/cards/param_card.dat");
        let mut vec: Vec<uux_aag::RustMG5> = vec![];
        for _ in 0..1000 {
            vec.push(integrand.clone());
        }
    }
}
