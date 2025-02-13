use colored::Colorize;

fn dot(p1: &[f64], p2: &[f64]) -> f64 {
    p1[0] * p2[0] - p1[1] * p2[1] - p1[2] * p2[2] - p1[3] * p2[3]
}

#[cfg(test)]
mod integrand_evaluation {
    use super::*;
    use rust_mg5::{uux_aa, uux_aaddx, uux_aag, uux_ddx};
    use rust_mg5::{MG5Integrand, MG5Parameters};

    #[test]
    fn evaluate_uux_aag() {
        // Initialize
        let card_path = "./standalone_sm_ma/Cards/param_card.dat";
        let mut mg5_integrand = uux_aag::RustMG5::init(card_path);
        //println!("====\n{}====\n",mg5_integrand.cout());

        // Set Momenta
        let moms = [
            [6500f64, 0f64, 0f64, 6500f64],
            [6500f64, 0f64, 0f64, -6500f64],
            [
                5961.499004870063,
                2202.624816443513,
                4934.899296802158,
                -2515.227222763681,
            ],
            [
                4733.170806863822,
                -238.259414094145,
                -4519.607957113826,
                1382.37730059228,
            ],
            [
                2305.330188266114,
                -1964.36540234937,
                -415.2913396883319,
                1132.8499221714,
            ],
        ];
        let moms_flat: Vec<f64> = moms.into_iter().flatten().collect();
        mg5_integrand.set_externals(&moms_flat);

        // Evaluate
        let res = mg5_integrand.evaluate();
        println!("res = {res:.5e}");
        println!("ninitial = {}", mg5_integrand.n_initials());
        println!("nexternal = {}", mg5_integrand.n_externals());
        println!("name = {}", mg5_integrand.name().bold().yellow());

        // Check result
        let res_check = 3.39417617222680e-09;
                assert!(
            (res - res_check) / res < 10e-16,
            "res = {res} different from res_check = {res_check} [delta:{:.2e}]",
            (res - res_check) / res
        );
        panic!();
    }

    #[test]
    fn evaluate_uux_aaddx() {
        // Initialize
        let card_path = "./standalone_sm_ma/Cards/param_card.dat";
        let mut mg5_integrand = uux_aaddx::RustMG5::init(card_path);
        //println!("====\n{}====\n",mg5_integrand.cout());

        // Set Momenta
        let moms = [
            [6500f64, 0f64, 0f64, 6500f64],
            [6500f64, 0f64, 0f64, -6500f64],
            [
                1154.368904125517,
                -287.2077175079854,
                520.8609598728074,
                -985.1232932745919,
            ],
            [
                4267.752487012273,
                -1349.569158468813,
                -3923.755484340945,
                994.0835099072848,
            ],
            [
                1979.957381165866,
                -1375.96737285329,
                -1269.777633056668,
                643.9019977181435,
            ],
            [
                5597.921227696344,
                3012.744248830089,
                4672.672157524804,
                -652.8622143508359,
            ],
        ];
        let moms_flat: Vec<f64> = moms.into_iter().flatten().collect();
        mg5_integrand.set_externals(&moms_flat);

        // Evaluate
        let res = mg5_integrand.evaluate();
        println!("res = {res:.5e}");
        println!("ninitial = {}", mg5_integrand.n_initials());
        println!("nexternal = {}", mg5_integrand.n_externals());
        println!("name = {}", mg5_integrand.name().bold().yellow());

        // Check result
        let res_check = 2.1933739244995207e-15;
        assert!(
            (res - res_check) / res < 10e-16,
            "res = {res} different from res_check = {res_check} [delta:{:.2e}]",
            (res - res_check) / res ,
        );
        panic!();
    }

    #[test]
    fn evaluate_uux_ddx() {
        // Initialize
        let card_path = "./standalone_sm_ma/Cards/param_card.dat";
        let mut mg5_integrand = uux_ddx::RustMG5::init(card_path);
        //println!("====\n{}====\n",mg5_integrand.cout());

        // Set Momenta
        let moms = [
            [6500f64, 0f64, 0f64, 6500f64],
            [6500f64, 0f64, 0f64, -6500f64],
            [
                6499.999999999997,
                1442.015697769826,
                5782.800263345577,
                -2594.188089101424,
            ],
            [
                6500.0,
                -1442.015697769826,
                -5782.800263345578,
                2594.188089101423,
            ],
        ];
        let moms_flat: Vec<f64> = moms.into_iter().flatten().collect();
        mg5_integrand.set_externals(&moms_flat);

        // Evaluate
        let res = mg5_integrand.evaluate();
        println!("res = {res:.5e}");
        println!("ninitial = {}", mg5_integrand.n_initials());
        println!("nexternal = {}", mg5_integrand.n_externals());
        println!("name = {}", mg5_integrand.name().bold().yellow());

        // Analytic expression
        // |M|^2 = 4/9 * 8/s^2 gs^4 (t^2+u^2)
        let p1 = &moms[0];
        let p2 = &moms[1];
        let q1 = &moms[2];
        let q2 = &moms[3];
        let mut analytic = 8.0 / dot(p1, p2).powi(2) * (dot(p1, q1).powi(2) + dot(p1, q2).powi(2));
        analytic *= 4.0 / 9.0 * mg5_integrand.G().powi(4);
        analytic /= 8.0; // Matching factor

        // Check result
        let res_check = 0.566450061160102f64;
        assert!(
            (res - res_check) / res < 10e-16,
            "res = {res} different from res_check = {res_check} [delta:{:.2e}]",
            (res - res_check) / res ,
        );
        assert!(
            (res - analytic) / res < 10e-16,
            "res = {res} different from analytic = {analytic}"
        );
        panic!();
    }
}
