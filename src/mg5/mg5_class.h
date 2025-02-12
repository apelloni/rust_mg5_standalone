#ifndef __MG5_CLASS__
#define __MG5_CLASS__

#include <cstdio>
#include <memory>
#include <sstream>
#include <string>
#include <vector>

#include "CPPProcess.h"
#include "Parameters_sm_ma.h"

namespace MG5_NAMESPACE {
class MG5Integrand {
public:
  _CPPProcess process;
  Parameters_sm_ma * parameters;
  // Constructor
  MG5Integrand();
  ~MG5Integrand();
  void init(const char *card_path);
  // void init();
  void set_momenta(const double *moms, const size_t size);
  double get_matrix_element();

  // Consts used for internal array limits
  size_t ninitial() const;
  size_t nexternal() const;
  size_t nprocesses() const;
  const double *get_masses() const;
  const std::string &get_name() const;

  // Get cout
  const std::string &read_cout();

  // Obtain parameters
  //
  // Model parameters independent of aS
  double get_mdl_WH() const;
  double get_mdl_WW() const;
  double get_mdl_WZ() const;
  double get_mdl_WT() const;
  double get_mdl_ymtau() const;
  double get_mdl_ymt() const;
  double get_mdl_ymb() const;
  double get_aS() const;
  double get_mdl_Gf() const;
  double get_aEWM1() const;
  double get_mdl_MH() const;
  double get_mdl_MZ() const;
  double get_mdl_MTA() const;
  double get_mdl_MT() const;
  double get_mdl_MB() const;
  double get_mdl_CKM3x3() const;
  double get_mdl_conjg_CKM1x1() const;
  double get_mdl_conjg_CKM3x3() const;
  double get_mdl_MZ_exp_2() const;
  double get_mdl_MZ_exp_4() const;
  double get_mdl_sqrt_2() const;
  double get_mdl_MH_exp_2() const;
  double get_mdl_aEW() const;
  double get_mdl_MW() const;
  double get_mdl_sqrt_aEW() const;
  double get_mdl_ee() const;
  double get_mdl_MW_exp_2() const;
  double get_mdl_sw2() const;
  double get_mdl_cw() const;
  double get_mdl_sqrt_sw2() const;
  double get_mdl_sw() const;
  double get_mdl_g1() const;
  double get_mdl_gw() const;
  double get_mdl_vev() const;
  double get_mdl_vev_exp_2() const;
  double get_mdl_lam() const;
  double get_mdl_yb() const;
  double get_mdl_yt() const;
  double get_mdl_ytau() const;
  double get_mdl_muH() const;
  double get_mdl_ee_exp_2() const;
  double get_mdl_sw_exp_2() const;
  double get_mdl_cw_exp_2() const;
  const std::vector<double> &get_mdl_complexi();
  const std::vector<double> &get_mdl_I1x33();
  const std::vector<double> &get_mdl_I2x33();
  const std::vector<double> &get_mdl_I3x33();
  const std::vector<double> &get_mdl_I4x33();

  // Model parameters dependent on aS
  double get_mdl_sqrt_aS() const;
  double get_G() const;
  double get_mdl_G_exp_2() const;

  // Model couplings independent of aS
  const std::vector<double> &get_GC_2();

  // Model couplings dependent on aS
  const std::vector<double> &get_GC_11();

private:
  std::vector<double *> p;
  std::vector<double> m;
  std::vector<double> c;
  std::string name;
  // cout redirect
  std::stringstream buffer;
  std::streambuf *old;
  std::string cout;
};

std::unique_ptr<MG5Integrand> new_mg5_integrand();
} // namespace MG5_NAMESPACE
#endif
