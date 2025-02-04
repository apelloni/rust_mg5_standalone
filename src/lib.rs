#[macro_use]
mod utils;

pub mod uux_aa;
pub mod uux_aaddx;
pub mod uux_aag;

pub trait MG5Integrand
where
    Self: Default,
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
